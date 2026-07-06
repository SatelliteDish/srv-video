use iced::{
    Element,
    widget::{
        column,
        text,
    },
};
use iced_video_player::{
    Video,
    VideoPlayer as IcedPlayer,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum VideoPlayerMessage {
    SetVideo(String),
}

#[derive(Debug)]
pub enum VideoPlayerStatus {
    Empty,
    Ready(Video),
}

pub struct VideoPlayer<'a> {
    player: Option<IcedPlayer<'a, VideoPlayerMessage>>,
    status: VideoPlayerStatus,
}

impl<'a> fmt::Debug for VideoPlayer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VideoPlayer")
            .field("status", &self.status)
            .field("player", &self.player.as_ref().map(|_| "Player"))
            .finish()
    }
}

impl<'a> VideoPlayer<'a> {
    pub fn new() -> Self {
        Self {
            player: None,
            status: VideoPlayerStatus::Empty,
        }
    }

    pub fn view(&self) -> Element<'_, VideoPlayerMessage> {
        let mut view = column([]);

        match &self.status {
            VideoPlayerStatus::Empty => {
                view = view.push(text("Select a video to get started"));
            },
            VideoPlayerStatus::Ready(video) => {
                view = view.push(IcedPlayer::new(&video));
            },
        }

        view.into()
    }

    pub fn update(&mut self, msg: VideoPlayerMessage) {
        match msg {
            VideoPlayerMessage::SetVideo(video_url) => {
                println!("{}",&video_url);
                println!("{}", &url::Url::parse(&video_url).unwrap());
                self.status = VideoPlayerStatus::Ready(
                    Video::new(&url::Url::parse(&video_url).unwrap()).unwrap()
                );
            },
        }
    }
}
