use clap::{
    Parser,
    Subcommand,
    Args,
};

mod follow;
use follow::{
    FollowCommandArgs,
    handle_follow_command,
};

mod following;
use following::{
    handle_following_args,
    FollowingArgs,
};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    Follow(FollowCommandArgs),
    Following(FollowingArgs),
}

#[derive(Debug, Args)]
struct WatchCommandArgs {
    url: String,
}



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommand::Follow(cmd) => handle_follow_command(cmd).await,
        CliCommand::Following(cmd) => handle_following_args(cmd).await,
    }
}
