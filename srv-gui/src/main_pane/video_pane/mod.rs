use iced::{
    Element,
    widget::{
        column, text,
    },
};
use iced_video_player::{
    Video,
};
use url::Url;
use srv_core::feed::VideoPost;

mod video_player;
use video_player::{
    VideoPlayer,
    VideoPlayerMessage,
};


#[derive(Debug, Clone)]
pub enum VideoPaneMessage {
    SetVideo(VideoPost),
    VideoPlayer(VideoPlayerMessage),
}

#[derive(Debug)]
pub struct VideoPane<'a> {
    pub video_data: Option<VideoPost>,
    pub player: VideoPlayer<'a>,
}

impl<'a> VideoPane<'a> {
    pub fn new() -> Self {
        Self {
            video_data: None,
            player: VideoPlayer::new(),
        }
    }

    pub fn view(&self) -> Element<'_, VideoPaneMessage> {
        let Self { video_data, player } = self;
        let title = video_data.as_ref().map(|v| v.meta.title.to_string()).unwrap_or("Select a video to get started".to_string());

        column![
            text(title),
            player.view().map(VideoPaneMessage::VideoPlayer),
        ].into()
    }

    pub fn update(&mut self, message: VideoPaneMessage) {
        match message {
            VideoPaneMessage::SetVideo(data) => {
                self.player.update(VideoPlayerMessage::SetVideo(data.src_set.get(0).unwrap().url.clone()));
                self.video_data = Some(data);
            },
            VideoPaneMessage::VideoPlayer(msg) => {},
        }
    }
}
