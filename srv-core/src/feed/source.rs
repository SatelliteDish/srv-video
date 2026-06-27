use rss::extension::Extension;
use mime::Mime;
use std::{
    collections::BTreeMap,
    str::FromStr,
};

use super::{
    EXTENSION_PREFIX,
    FeedError,
};

#[derive(Debug,Clone)]
pub struct VideoSource {
    pub url: String,
    // If none provided, endpoint is dynamic and takes a "d" query param that takes an "x" separated
    // string in the form of "<WIDTH>x<HEIGHT>"
    pub dimensions: Option<(u64, u64)>,
    pub mime: Mime,
}

impl VideoSource {
    pub fn get_qualified_name() -> String {
        format!("{EXTENSION_PREFIX}:source")
    }
}

impl From<VideoSource> for BTreeMap<String,String> {
    fn from(value: VideoSource) -> Self {
        let mut out = BTreeMap::from([
                ("url".to_string(), value.url),
                ("mime".to_string(), value.mime.to_string()),
        ]);
        if let Some((width,height)) = value.dimensions {
            out.insert("width".to_string(), width.to_string());
            out.insert("height".to_string(), height.to_string());
        }

        out
    }
}

impl From<VideoSource> for Extension {
    fn from(value: VideoSource) -> Self {
        Self {
            name: VideoSource::get_qualified_name(),
            value: None,
            attrs: value.into(),
            children: BTreeMap::new(),
        }
    }
}

impl TryFrom<BTreeMap<String, String>> for VideoSource {
    type Error = FeedError;
    fn try_from(value: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let url = value.get("url")
            .ok_or(FeedError::MissingRequiredField(format!("url")))?
            .to_string();
        let mime_str = value.get("mime")
            .ok_or(FeedError::MissingRequiredField(format!("mime")))?;
        let mime: Mime = Mime::from_str(&mime_str)
            .map_err(|_| FeedError::InvalidField { field: format!("mime"), value: mime_str.to_string() })?;

        let dimensions = if let Some(width_str) = value.get("width") {
            if let Some(height_str) = value.get("height") {
                Ok(Some((
                    width_str.parse::<u64>()
                        .map_err(|_| FeedError::InvalidField { field: format!("width"), value: format!("{}",width_str) })?,
                    height_str.parse::<u64>()
                        .map_err(|_| FeedError::InvalidField { field: format!("height"), value: format!("{}",height_str) })?,
                )))
            } else {
                Err(FeedError::MissingRequiredField(format!("height")))
            }
        } else if let Some(_) = value.get("height") {
            Err(FeedError::MissingRequiredField(format!("width")))
        } else {
            Ok(None)
        }?;

        Ok(VideoSource {
            url,
            mime,
            dimensions,
        })
    }
}
