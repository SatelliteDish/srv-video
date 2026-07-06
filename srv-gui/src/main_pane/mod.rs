use iced::{
    Element,
    widget::{
        container,
    },
};
use srv_core::feed::{
    Feed,
    VideoPost,
};

mod start;
use srv_watch_core::feed::FollowedFeed;
use start::{
    StartPane,
    StartPaneMessage,
};

mod feed_pane;
use feed_pane::{
    FeedPane,
    FeedPaneMessage,
};

use crate::main_pane::{video_pane::{VideoPane,VideoPaneMessage}};

mod video_pane;

#[derive(Debug, Clone)]
pub enum MainPaneState {
    Start,
    Feed,
    Video,
}

#[derive(Debug, Clone)]
pub enum MainPaneMessage {
    ShowStart,
    ShowFeed(Feed),
    StartPane(StartPaneMessage),
    FeedPane(FeedPaneMessage),
    VideoPane(VideoPaneMessage),
}


#[derive(Debug)]
pub struct MainPane<'a> {
    state: MainPaneState,
    start_pane: StartPane,
    feed_pane: FeedPane,
    video_pane: VideoPane<'a>,
}

impl<'a> MainPane<'a> {
    pub fn new() -> Self {
        Self {
            state: MainPaneState::Start,
            start_pane: StartPane::new(),
            feed_pane: FeedPane::new(),
            video_pane: VideoPane::new(),
        }
    }

    pub fn view(&self) -> Element<'_, MainPaneMessage> {
        match &self.state {
            MainPaneState::Start => self.start_pane.view().map(MainPaneMessage::StartPane),
            MainPaneState::Feed => self.feed_pane.view().map(MainPaneMessage::FeedPane),
            MainPaneState::Video => self.video_pane.view().map(MainPaneMessage::VideoPane),
            _ => todo!(),
        }
    }

    pub fn update(&mut self, msg: MainPaneMessage) {
        match msg {
            MainPaneMessage::ShowStart => self.state = MainPaneState::Start,
            MainPaneMessage::ShowFeed(feed) => {
                self.state = MainPaneState::Feed;
                self.feed_pane.show_feed(feed);
            },
            MainPaneMessage::StartPane(msg) => self.start_pane.update(msg),
            MainPaneMessage::FeedPane(msg) => {
                match msg {
                    FeedPaneMessage::VideoSelected(data) => {
                        self.update(MainPaneMessage::VideoPane(VideoPaneMessage::SetVideo(data)));
                        self.state = MainPaneState::Video;
                    },
                }
            },
            MainPaneMessage::VideoPane(msg) => self.video_pane.update(msg),
        }
    }
}
