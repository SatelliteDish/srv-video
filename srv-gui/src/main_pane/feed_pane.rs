use iced::{
    Element,
    widget::{
        button,
        column,
        text,
    },
};
use srv_core::feed::{Feed, VideoMetadata, VideoPost};



#[derive(Debug,Clone)]
pub enum FeedPaneMessage {
    VideoSelected(VideoPost),
}

#[derive(Debug,Clone)]
pub enum FeedPaneStatus {
    Waiting,
    Ready(Feed),
}

#[derive(Debug,Clone)]
pub struct FeedPane {
    status: FeedPaneStatus,
}

impl FeedPane {
    pub fn new() -> Self {
        Self {
            status: FeedPaneStatus::Waiting,
        }
    }

    pub fn view(&self) -> Element<'_, FeedPaneMessage> {

        match &self.status {
            FeedPaneStatus::Waiting => todo!(),
            FeedPaneStatus::Ready(feed) => {
                let Feed { title, description, link, image, videos } = feed;
                let video_elements: Vec<Element<'_, FeedPaneMessage>> = videos.into_iter()
                    .map(video_display)
                    .collect();

                column![
                    text(title),
                    text(description),
                ].extend(video_elements).into()
            }
        }
    }

    pub fn show_feed(&mut self, feed: Feed) {
        self.status = FeedPaneStatus::Ready(feed);
    }
}


fn video_display(video: &VideoPost) -> Element<'_, FeedPaneMessage> {
    let VideoMetadata { title, description, .. } = &video.meta;
    button(column![
        text(title),
        text(description.as_deref().unwrap_or("")),
    ])
        .on_press(FeedPaneMessage::VideoSelected(video.clone()))
        .into()
}
