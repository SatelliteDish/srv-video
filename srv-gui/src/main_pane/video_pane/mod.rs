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
    pub video_data: VideoPost,
    pub player: VideoPlayer<'a>,
}

impl<'a> VideoPane<'a> {
    pub fn new(video_data: VideoPost) -> Self {
        let mut player = VideoPlayer::new();
        player.update(VideoPlayerMessage::SetVideo(video_data.src_set.get(0).unwrap().url.clone()));

        Self {
            video_data,
            player,
        }
    }

    pub fn view(&self) -> Element<'_, VideoPaneMessage> {
        let Self { video_data, player } = self;
        let title = video_data.meta.title.to_string();

        column![
            text(title),
            player.view().map(VideoPaneMessage::VideoPlayer),
        ].into()
    }

    pub fn update(&mut self, message: VideoPaneMessage) {
        match message {
            VideoPaneMessage::SetVideo(data) => {
                self.video_data = data;
            },
            VideoPaneMessage::VideoPlayer(msg) => {},
        }
    }
}
