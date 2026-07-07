use iced::{
    Element,
    widget::{
        column, text,
    },
};
use srv_core::feed::VideoPost;

mod video_player;
use video_player::{
    VideoPlayer,
    VideoPlayerMessage,
};


#[derive(Debug, Clone)]
pub enum VideoPaneMessage {
    VideoPlayer(VideoPlayerMessage),
}

#[derive(Debug)]
pub struct VideoPane {
    pub video_data: VideoPost,
    pub player: VideoPlayer,
}

impl VideoPane {
    pub fn new(video_data: VideoPost) -> Self {
        Self {
            player: VideoPlayer::new(&video_data.src_set.get(0).unwrap().url),
            video_data,
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
            VideoPaneMessage::VideoPlayer(msg) => self.player.update(msg),
        }
    }
}
