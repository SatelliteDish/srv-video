use clap::{
    Subcommand,
    Args,
};
use cli_table::{
    Cell,
    Table,
    format::Align,
    print_stdout,
};
use srv_core::feed::VideoPost;
use srv_watch_core::{
    database::follow::{
        Subscription,
    },
    feed::{
        get_feed,
        unfollow,
    },
};



#[derive(Debug, Args)]
pub struct FollowingArgs {
    #[command(subcommand)]
    command: Option<FollowingCommand>,
}

#[derive(Debug, Subcommand)]
pub enum FollowingCommand {
    Videos(FollowingVideosArgs),
    Remove(FollowingRemoveArgs)
}

#[derive(Debug, Args)]
pub struct FollowingVideosArgs {
    feed_id: u32,
    #[command(subcommand)]
    command: Option<FollowingVideosCommand>,
}

#[derive(Debug, Args)]
pub struct FollowingRemoveArgs {
    feed_id: u32,
}


#[derive(Debug, Subcommand)]
pub enum FollowingVideosCommand {
    Play(FollowingVideosPlayArgs),
}

#[derive(Debug, Args)]
pub struct FollowingVideosPlayArgs {
    video_id: u32,
}



pub async fn handle_following_args(command: FollowingArgs) -> std::io::Result<()> {
    let following = srv_watch_core::feed::get_following();

    if let Some(cmd) = command.command {
        handle_following_command(cmd, following).await
    } else {
        let table = following
            .into_iter()
            .enumerate()
            .map(|(i,sb)| vec![
                format!("{i}").cell(),
                sb.feed.title().cell().align(Align::Center),
                sb.feed.description.cell(),
                sb.url.cell(),
            ]).table();
        print_stdout(table)?;

        Ok(())
    }
}

async fn handle_following_command(command: FollowingCommand, following: Vec<Subscription>) -> std::io::Result<()> {
    match command {
        FollowingCommand::Videos(cmd) => handle_following_videos_args(cmd, following).await,
        FollowingCommand::Remove(cmd) => handle_following_remove_args(cmd, following).await,
    }
}

async fn handle_following_videos_args(command: FollowingVideosArgs, following: Vec<Subscription>) -> std::io::Result<()> {
    let picked: &Subscription = following.get(command.feed_id as usize).unwrap();
    let feed = get_feed(&picked.url).await.unwrap();
    if let Some(items) = feed.items {
        let table = items.iter()
            .enumerate()
            .map(|(i,vd)| vec![
                format!("{i}").cell(),
                vd.meta.title.clone().cell(),
                vd.meta.description.clone().unwrap_or("".to_string()).cell(),
            ]).table();
        print_stdout(table)?;

        if let Some(args) = command.command {
            handle_following_videos_command(args, items).await?;
        }
    }
    Ok(())
}

async fn handle_following_remove_args(command: FollowingRemoveArgs, following: Vec<Subscription>) -> std::io::Result<()> {
    let picked = following.get(command.feed_id as usize).unwrap();
    unfollow(&picked.url);
    Ok(())
}

async fn handle_following_videos_command(command: FollowingVideosCommand, videos: Vec<VideoPost>) -> std::io::Result<()> {
    match command {
        FollowingVideosCommand::Play(args) => handle_following_videos_play_command(args, videos).await,
    }
}


async fn handle_following_videos_play_command(command: FollowingVideosPlayArgs, videos: Vec<VideoPost>) -> std::io::Result<()> {
    let selected = videos.get(command.video_id as usize).unwrap();
    let url = &selected.src_set.get(0)
        .unwrap().url;
    open::that_detached(url)?;
    Ok(())
}
