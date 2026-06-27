use clap::{
    Parser,
    Subcommand,
    Args,
};
use srv_core::feed::{
    Feed,
    VideoPost,
    VideoSource,
    VideoMetadata,
};
use srv_host_core::{
    database,
    rss,
};
use mime::Mime;
use std::{
    collections::HashMap,
    io,
    str::FromStr as _,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    #[command(subcommand)]
    Video(VideoCommand),
    #[command(subcommand)]
    Feed(FeedCommand),
}

#[derive(Debug, Subcommand)]
enum FeedCommand {
    Create(CreateFeedArgs)
}

#[derive(Debug, Args)]
struct CreateFeedArgs {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    link: String,
    #[arg(short, long)]
    description: String,
    #[arg(short, long)]
    image: Option<String>,
}


#[derive(Debug, Subcommand)]
enum VideoCommand {
    Add(VideoAddCommand)
}

#[derive(Debug, Args)]
struct VideoAddCommand {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    link: String,
    #[arg(short, long)]
    description: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommand::Video(cmd) => handle_video_cmd(cmd).await,
        CliCommand::Feed(cmd) => handle_feed_cmd(cmd).await,
    };

    Ok(())
}

async fn handle_feed_cmd(cmd: FeedCommand) {
    match cmd {
        FeedCommand::Create(args) => {
            rss::save_feed(&Feed {
                title: args.title,
                link: args.link,
                description: args.description,
                image: args.image,
                items: Some(vec![]),
            }).await.unwrap();
        },
    }
}


async fn handle_video_cmd(cmd: VideoCommand) {
    match cmd {
        VideoCommand::Add(args) => {
            let db = database::get_db_connection().unwrap();
            let id = database::videos::register_video(&db, &args.link).await.unwrap();

            rss::post_to_feed(VideoPost {
                meta: VideoMetadata {
                    title: args.title,
                    thumbnail: None,
                    description: args.description,
                    categories: vec![],
                    references: HashMap::new(),
                    related: vec![],
                },
                comment_url: None,
                rating_url: None,
                src_set: vec![VideoSource {
                    url: format!("http://10.0.0.85/stream/{id}"),
                    mime: Mime::from_str("video/mp4").unwrap(),
                    dimensions: None,
                }],
            }).await.unwrap();
        }
    }
}
