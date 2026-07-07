use iced::{
    Element,
};
use srv_core::feed::{
    Feed,
};

mod start;
use start::{
    StartPane,
    StartPaneMessage,
};

mod feed_pane;
use feed_pane::{
    FeedPane,
    FeedPaneMessage,
};


mod video_pane;
use video_pane::{
    VideoPane,
    VideoPaneMessage,
};

#[derive(Debug, Clone)]
pub enum MainPaneMessage {
    ShowStart,
    ShowFeed(Feed),
    StartPane(StartPaneMessage),
    FeedPane(FeedPaneMessage),
    VideoPane(VideoPaneMessage),
}

#[derive(Debug)]
pub enum PaneState {
    Start(StartPane),
    Feed(FeedPane),
    Video(VideoPane),
}

#[derive(Debug)]
pub struct MainPane {
    pane: PaneState,
}

impl MainPane {
    pub fn new() -> Self {
        Self {
            pane: PaneState::Start(StartPane::new()),
        }
    }

    pub fn view(&self) -> Element<'_, MainPaneMessage> {
        match &self.pane {
            PaneState::Start(pane) => pane.view().map(MainPaneMessage::StartPane),
            PaneState::Feed(pane) => pane.view().map(MainPaneMessage::FeedPane),
            PaneState::Video(pane) => pane.view().map(MainPaneMessage::VideoPane),
        }
    }

    pub fn update(&mut self, msg: MainPaneMessage) {
        match msg {
            MainPaneMessage::ShowStart => self.pane = PaneState::Start(StartPane::new()),
            MainPaneMessage::ShowFeed(feed) => {
                let mut pane = FeedPane::new();
                pane.show_feed(feed);
                self.pane = PaneState::Feed(pane);
            },
            MainPaneMessage::StartPane(msg) => {
                if let PaneState::Start(pane) = &self.pane {
                    pane.update(msg);
                }
            },
            MainPaneMessage::FeedPane(msg) => {
                match msg {
                    FeedPaneMessage::VideoSelected(data) => {
                        self.pane = PaneState::Video(VideoPane::new(data));
                    },
                }
            },
            MainPaneMessage::VideoPane(msg) => {
                if let PaneState::Video(pane) = &mut self.pane {
                    pane.update(msg);
                }
            },
        }
    }
}
