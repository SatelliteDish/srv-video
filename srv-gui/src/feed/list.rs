use iced::{
    Element,
    Length,
    widget::{
        column,
        text,
        button,
        container,
        button::text as text_button,
        container::bordered_box,
    },
};
use srv_watch_core::feed::FollowedFeed;

#[derive(Debug, Clone)]
pub enum FeedListMessage {
    Select(FollowedFeed),
}

#[derive(Debug)]
pub struct FeedList {
    pub feed: Vec<FollowedFeed>,
    pub selected: Option<FollowedFeed>,
}

impl FeedList {
    pub fn new(feed: Vec<FollowedFeed>) -> Self {
        Self {
            feed,
            selected: None,
        }
    }

    pub fn view(&self) -> Element<'_, FeedListMessage> {
        container(column(
                self.feed
                    .iter()
                    .map(|f| {
                        let selected = if let Some(sel) = &self.selected {
                            (sel.title() == f.title()) &&
                            (sel.description() == f.description())
                        } else { false };
                        feed_list_item(f, selected).into()
                    })
        ))
            .style(bordered_box)
            .width(Length::Fixed(250.0))
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, msg: FeedListMessage) {
        match msg {
            FeedListMessage::Select(feed) => self.selected = Some(feed),
        }
    }
}

pub fn feed_list_item(feed: &FollowedFeed, active: bool) -> Element<'_, FeedListMessage> {
    let title = feed.title();
    let description = feed.description();

    button(column![
        text(title),
        text(description),
    ])
        .on_press(FeedListMessage::Select(feed.clone()))
        .style(move|style, status| {
            text_button(style, if active { button::Status::Pressed } else { status })
        })
        .into()
}
