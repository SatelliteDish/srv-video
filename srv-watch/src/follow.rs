use clap::Args;
use srv_watch_core::feed::{
    follow_feed,
    get_feed,
};

#[derive(Debug, Args)]
pub struct FollowCommandArgs {
    pub url: String
}

pub async fn handle_follow_command(args: FollowCommandArgs) -> std::io::Result<()> {
    let FollowCommandArgs { url } = args;
    let feed = get_feed(&url).await.unwrap();
    follow_feed(&url, &feed).await;
    println!("You are now subscribed to: {}", feed.title());

    Ok(())
}
