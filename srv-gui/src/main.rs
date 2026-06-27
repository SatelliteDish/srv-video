use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
    Theme,
    widget::{
        column,
        row,
        container,
        button,
        text,
    },
};
use srv_core::feed::Feed;
use srv_watch_core::feed::get_following;

mod icon;

#[derive(Debug, Clone, Default)]
struct SrvState {
    feed: Vec<Feed>,
}

pub fn update(state: &mut SrvState, msg: Message) -> Task<Message> {
    match msg {
        Message::Close => iced::window::latest().and_then(iced::window::close),
    }
}

#[derive(Debug,Clone)]
enum Message {
    Close,
}

fn main() -> iced::Result {
    iced::application(|| SrvState {
        feed: get_following().into_iter().map(|f| Feed {
            title: f.feed.title,
            ..Default::default()
        }).collect(),
    }, update, view)
        .theme(Theme::GruvboxDark)
        .title("SrvGui")
        .font(icon::FONT)
        .run()

}

fn top_bar_view<'a>() -> Element<'a, Message> {
    row![
        container(text("SrvGui"))
            .width(Fill)
            .align_x(Center),
        button(icon::x()).on_press(Message::Close),
    ].width(Fill).into()
}


fn view(state: &SrvState) -> Element<'_, Message> {
        column![
            top_bar_view(),
            feed_list(state),
        ].into()
}


fn feed_list(state: &SrvState) -> Element<'_, Message> {
    column(
        state.feed.iter()
            .map(|feed| text(&feed.title).into())
    ).into()
}
