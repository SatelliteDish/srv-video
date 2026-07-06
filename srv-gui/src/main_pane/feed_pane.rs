use iced::{
    Alignment, Element, Length, widget::{
        Column, button, column, text,
    },
};
use srv_core::feed::{Feed, VideoPost};


#[derive(Debug,Clone)]
pub enum FeedPaneMessage {
    VideoSelected(VideoPost),
}

#[derive(Debug,Clone)]
pub struct FeedPane {
    pub feed: Option<Feed>,
}

impl FeedPane {
    pub fn new() -> Self {
        Self {
            feed: None,
        }
    }

    pub fn view(&self) -> Element<'_, FeedPaneMessage> {
        let title = self.feed.clone().map(|feed| feed.title).unwrap_or("Select a feed".to_string());
        let description = self.feed.clone().map(|feed| feed.description).unwrap_or("".to_string());
        let videos = self.feed.clone().map(|feed| feed.videos).unwrap_or(vec![]);
        let video_elements: Vec<Element<'_, FeedPaneMessage>> = videos.into_iter()
            .map(|vd| video_display(vd))
            .collect();

        column![
            text(title),
            text(description),
        ].extend(video_elements).into()
    }

    pub fn show_feed(&mut self, feed: Feed) {
        self.feed = Some(feed);
    }
}


fn video_display<'a>(video: VideoPost) -> Element<'a, FeedPaneMessage> {
    button(column![
        text(video.meta.clone().title),
        text(video.meta.clone().description.unwrap_or(String::new())),
    ])
        .on_press(FeedPaneMessage::VideoSelected(video))
        .into()
}
