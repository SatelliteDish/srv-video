use srv_core::feed::Feed;
use thiserror::Error;
use rss::Channel;

use crate::database::{
    follow::{
        self,
        Subscription,
    },
    get_db_connection
};


#[derive(Debug, Error)]
pub enum FeedError {
    #[error("{0}")]
    NetworkErr(#[from]reqwest::Error),
    #[error("{0}")]
    RssErr(#[from]rss::Error),
}

pub async fn get_feed(url: &str) -> Result <Feed, FeedError> {
    let feed = reqwest::get(url).await?
        .bytes().await?;

    let channel = Channel::read_from(&feed[..])?;
    Ok(channel.into())
}

pub async fn follow_feed(url: &str, feed: &Feed) {
    let db = get_db_connection();

    follow::follow(&db, url, feed).await.unwrap();
}

pub fn get_following() -> Vec<Subscription> {
    let db = get_db_connection();
    follow::query_following(&db)
        .unwrap()
}
