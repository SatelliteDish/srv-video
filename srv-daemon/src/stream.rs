use actix_web::{
    HttpRequest,
    HttpResponse,
    Result,
    error,
    http::header::{
        ACCEPT_RANGES,
        CONTENT_LENGTH,
        CONTENT_RANGE,
        IF_RANGE,
        LAST_MODIFIED,
        RANGE,
    }, web
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::{
    fs::File,
};
use tokio_util::io::ReaderStream;
use futures::TryStreamExt;
use slog::{
    Logger,
    o,
    warn,
    error,
    crit,
};
use chrono::DateTime;
use chrono::{
    Utc,
};
use tap::{TapFallible as _, TapOptional};
use srv_host_core::database::{self, get_db_connection};
use std::{
    fs::Metadata, io::SeekFrom, path::PathBuf, time::UNIX_EPOCH
};


const RANGE_START: &str = "bytes=";
const RANGE_START_LEN: usize = RANGE_START.len();


async fn get_file_path(id: &str) -> PathBuf {
    let db = get_db_connection().unwrap();
    let video = database::videos::query_videos(&db).await.unwrap()
        .into_iter().find(|f| id == f.id.as_str()).unwrap();

    PathBuf::from(video.uri)
}


pub async fn stream(root_log: web::Data<Logger>, id: web::Path<String>, req: HttpRequest) -> Result<HttpResponse> {
    let log = root_log.new(o!{"route" => format!("/stream/{id}")});

    let path = get_file_path(&id).await;
    warn!(log, "VID PATH: {}", &path.to_string_lossy());
    // Open file async
    let file = File::open(&path)
        .await
        .tap_err(|e| warn!(log, "Could not open file at {}:\n{e}", &path.to_string_lossy()))
        .map_err(actix_web::error::ErrorNotFound)?; // Return 404

    // Parse relevant metadata
    let metadata = file.metadata()
        .await
        .tap_err(|e| crit!(log, "Could not retrieve metadata of file \"{}\"\n{e}", &path.to_string_lossy()))
        .map_err(error::ErrorInternalServerError)?;

    let file_modified: DateTime<Utc> = {
        let mod_raw = metadata.modified()
            .tap_err(|e| warn!(log, "Could not get file metadata for {}:\n{e}", &path.to_string_lossy()))
            .map_err(error::ErrorInternalServerError)?;

        let mod_epoch = mod_raw.duration_since(UNIX_EPOCH)
            .tap_err(|e| error!(log, "Could not get epoch time\n{e}"))
            .map_err(error::ErrorInternalServerError)?;

        DateTime::from_timestamp(
            mod_epoch.as_secs().cast_signed(),
            0,
        ).ok_or(error::ErrorInternalServerError("Internal Server Error"))
            .tap_err(|e| error!(log, "Could not parse file time as DateTime:\n{e}"))?
    };

    // Parse headers
    let headers = req.headers();
    let range_hdr = headers.get(RANGE);

    // Decide whether to stream full file or range
    // Stream range if:
    //   - RANGE header exists
    //   - No IF RANGE header
    //   - IF RANGE timestamp matches
    if let Some(hdr_val) = range_hdr {
        // Stream ranges verified in handler, syntax verified here
        let range_str = hdr_val.to_str()
            .tap_err(|e| warn!(log, "unparseable Range header:\n{e}"))
            .map_err(error::ErrorBadRequest)?;

        if &range_str[..RANGE_START_LEN] != RANGE_START {
            return Err(
                error::ErrorBadRequest("Only byte ranges supported")
            );
        }

        // Parse range
        let mut split = range_str[RANGE_START_LEN..].split("-");
        let start = split.next()
            .tap_none(|| warn!(log, "Range header missing start"))
            .ok_or(error::ErrorBadRequest("Invalid range, start required"))?
            .parse::<u64>()
            .tap_err(|e| warn!(log, "Range header start not parseable as u64:\n{e}"))
            .map_err(error::ErrorBadRequest)?;
        let end = split.next()
            .filter(|s| !s.is_empty())
            .map(|end| end.parse::<u64>())
            .map_or(Ok(None), |v| v.map(Some)) // Convert Option<Result> to Result<Option>
            .tap_err(|e| warn!(log, "Could not parse end of range as u64:\n{e}"))
            .map_err(error::ErrorBadRequest)?;

        if let Some(if_range_hdr) = headers.get(IF_RANGE) { // If Range exists
            let if_range_str = if_range_hdr.to_str()
                .tap_err(|e| warn!(log, "could not parse If Range header as str:\n{e}"))
                .map_err(error::ErrorBadRequest)?;

            let req_modified_date = DateTime::parse_from_rfc2822(if_range_str)
                .map_err(error::ErrorBadRequest)?;
            if file_modified== req_modified_date { // And matches
                return stream_range(log, file, metadata, start, end, file_modified).await;
            }

            // Fall through to full streaming
        } else { // No If Range exists
            return stream_range(log, file, metadata, start, end, file_modified).await;
        }
    }


    // Stream full if:
    //   - No RANGE header
    //   - IF RANGE does timestamp doesn't match

    stream_full(file, metadata, file_modified).await
}

async fn stream_full(file: File, metadata: Metadata, modified: DateTime<Utc>) -> Result<HttpResponse> {
    let stream = ReaderStream::new(file)
        .map_err(error::ErrorInternalServerError);

    Ok(
        HttpResponse::Ok()
            .content_type("video/mp4")
            .insert_header((CONTENT_LENGTH, metadata.len()))
            .insert_header((ACCEPT_RANGES, "bytes"))
            .insert_header((LAST_MODIFIED, modified.to_rfc2822()))
            .streaming(stream)
    )
}

async fn stream_range(
    log: Logger,
    mut file: File,
    metadata: Metadata,
    start: u64,
    end_opt: Option<u64>,
    modified: DateTime<Utc>,
) -> Result<HttpResponse> {
    let end = end_opt.unwrap_or(metadata.len() - 1);
    let size = end - start;

    file.seek(SeekFrom::Start(start)).await
        .tap_err(|e| error!(log, "Requested start beyond max:\n{e}"))
        .map_err(error::ErrorInternalServerError)?;
    file = file.take(size).into_inner();


    let stream = ReaderStream::new(file)
        .map_err(error::ErrorInternalServerError);
    Ok(
        HttpResponse::PartialContent()
            .content_type("video/mp4")
            .insert_header((CONTENT_LENGTH, size + 1))
            .insert_header((ACCEPT_RANGES, "bytes"))
            .insert_header((CONTENT_RANGE, format!("bytes {start}-{end}/{}", metadata.len())))
            .insert_header((LAST_MODIFIED, modified.to_rfc2822()))
            .streaming(stream)
    )
}
