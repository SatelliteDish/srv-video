use actix_web::{
    App,
    HttpServer,
    Responder,
    web,
};
use slog::{
    Drain,
    info,
    o,
};
use std::fs::read_to_string;

mod stream;
mod config;
use config::get_config;

async fn feed() -> impl Responder {
    let mut dir = srv_host_core::get_data_dir().unwrap();
    dir.push("feed.xml");

    read_to_string(dir)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config(None);

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());


    info!(_log,"Listening on {}:{}", config.host(), config.port());
    let log_data = web::Data::new(_log);
    HttpServer::new(move || {
        App::new()
            .app_data(log_data.clone())
            .route("/feed.xml", web::get().to(feed))
            .route("/stream/{path}", web::get().to(stream::stream))
    })
    .bind((config.host(), config.port()))?
    .run()
    .await
}
