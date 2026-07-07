use std::time::Duration;

use iced::{
    Alignment, Element, Length, widget::{
        Slider, button, column, row,
    },
};
use iced_video_player::{
    Video, VideoPlayer as IcedPlayer,
};
use crate::icon;

#[derive(Debug, Clone)]
pub enum VideoPlayerMessage {
    Play,
    Pause,
    Seek(f64),
    SeekRelease,
}

#[derive(Debug)]
pub struct VideoPlayer {
    video: Video,
    position: f64,
    dragging: bool,
}


impl VideoPlayer {
    pub fn new(video_url: &str) -> Self {
        Self {
            video: Video::new(&url::Url::parse(video_url).unwrap()).unwrap(),
            position: 0.0,
            dragging: false,
        }
    }

    fn pause(&mut self) {
        self.video.set_paused(true);
    }

    fn unpause(&mut self) {
        self.video.set_paused(false);
    }

    pub fn view(&self) -> Element<'_, VideoPlayerMessage> {
        column![
            IcedPlayer::new(&self.video),
            self.control_bar(),
        ].into()
    }

    fn control_bar(&self) -> Element<'_, VideoPlayerMessage> {
        row![
            self.pause_button(),
            Slider::new(
                0.0..=self.video.duration().as_secs_f64(),
                self.position,
                VideoPlayerMessage::Seek,
            )
                .step(0.1)
                .on_release(VideoPlayerMessage::SeekRelease),
        ]
            .height(Length::Fixed(60.0))
            .align_y(Alignment::Center)
            .into()
    }

    fn pause_button(&self) -> Element<'_, VideoPlayerMessage> {
        if self.video.paused() {
            button(
                icon::play()
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0))
            )
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0))
                .on_press(VideoPlayerMessage::Play)
        } else {
            button(
                    icon::pause()
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0))
                )
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0))
                .on_press(VideoPlayerMessage::Pause)
        }.into()
    }

    pub fn update(&mut self, msg: VideoPlayerMessage) {
        match msg {
            VideoPlayerMessage::Play => self.unpause(),
            VideoPlayerMessage::Pause => self.pause(),
            VideoPlayerMessage::Seek(pos) => {
                self.pause();
                self.dragging = true;
                self.position = pos;
            },
            VideoPlayerMessage::SeekRelease => {
                self.dragging = false;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("Failed to seek");
                self.unpause();
            },
        }
    }
}
