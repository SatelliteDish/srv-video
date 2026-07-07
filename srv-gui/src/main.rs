use iced::{
    Element,
    Task,
    Theme,
    widget::{
        column,
        row,
    },
};
use srv_core::feed::Feed;
use srv_watch_core::feed::{
    get_following,
    FollowedFeed,
};

mod icon;

mod main_pane;
use main_pane::{
    MainPane,
    MainPaneMessage,
};

mod feed;
use feed::{
    FeedListMessage,
    FeedList,
};

mod top_bar;
use top_bar::{
    TopBarAction,
    TopBar,
};

#[derive(Debug)]
struct SrvState {
    top_bar: TopBar,
    feed: FeedList,
    main_pane: MainPane,
}

impl SrvState {
    pub fn new() -> Self {
        Self {
            top_bar: TopBar::new(),
            feed: FeedList::new(get_following()),
            main_pane: MainPane::new(),
        }
    }
}


#[derive(Debug,Clone)]
enum Message {
    TopBar(TopBarAction),
    FeedList(FeedListMessage),
    MainPane(MainPaneMessage),
}



fn main() -> iced::Result {
    iced::application(SrvState::new, update, view)
        .theme(Theme::GruvboxDark)
        .title("SrvGui")
        .font(icon::FONT)
        .run()

}

fn view(state: &SrvState) -> Element<'_, Message> {
        column![
            state.top_bar.view().map(Message::TopBar),
            row![
                state.feed.view().map(Message::FeedList),
                state.main_pane.view().map(Message::MainPane),
            ],
        ].into()
}

async fn get_fresh_feed(mut feed: FollowedFeed) -> Feed {
    feed.get_fresh_feed().await
}

fn update(state: &mut SrvState, msg: Message) -> Task<Message> {
    match msg {
        Message::TopBar(msg) => handle_top_bar(msg),
        Message::FeedList(FeedListMessage::Select(feed)) => {
            Task::perform(
                get_fresh_feed(feed),
                |f| Message::MainPane(MainPaneMessage::ShowFeed(f))
            )
        },
        Message::FeedList(msg) => {
            state.feed.update(msg).into()
        },
        Message::MainPane(msg) => state.main_pane.update(msg).into(),
    }
}

fn handle_top_bar(action: TopBarAction) -> Task<Message> {
    match action {
        TopBarAction::Close => iced::window::latest().and_then(iced::window::close),
    }
}
