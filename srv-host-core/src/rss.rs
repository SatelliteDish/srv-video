use rss::{
    Channel,
};
use thiserror::Error;
use std::{
    fs::{
        self,
        File,
        OpenOptions,
    },
    io::{self, BufReader, Seek},
};
use srv_core::feed::{
    Feed,
    VideoPost,
};
use std::collections::BTreeMap;

use super::{
    get_data_dir,
    IoErr,
};

const FEED_FILE: &str = "feed.xml";

#[derive(Debug, Error)]
pub enum RssErr {
    #[error("{0}")]
    IoError(#[from]IoErr),
    #[error("{0}")]
    RssError(#[from]rss::Error),
}


pub async fn save_feed(feed: &Feed) -> Result<(), RssErr> {
    let chan = Channel {
        title: feed.title.to_string(),
        link: feed.link.to_string(),
        description: feed.description.to_string(),
        namespaces: BTreeMap::from([
            (
                "srv".to_string(),
                "https://srv-video.org/ns/1.0".to_string(),
            )
        ]),
        // TODO: add image support
        ..Default::default()
    };

    let mut path = get_data_dir()?;
    // TODO add support for multiple feeds
    path.push(FEED_FILE);

    if fs::exists(&path).map_err(IoErr::from)? {
        Err(IoErr::from(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{} already exists", path.to_string_lossy()),
        )).into())
    } else {
        let file = File::create(&path).map_err(IoErr::from)?;
        chan.write_to(file)?;
        Ok(())
    }
}
pub async fn post_to_feed(item: VideoPost) -> Result<(), RssErr> {
    let mut path = get_data_dir()?;
    path.push(FEED_FILE);

    if fs::exists(&path).map_err(IoErr::from)? {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(IoErr::from)?;
        let stream = BufReader::new(&file);


        let mut chan = Channel::read_from(stream)?;
        let mut items = chan.items().to_vec();
        items.push(item.into());
        chan.set_items(items);

        file.seek(io::SeekFrom::Start(0))
            .map_err(IoErr::from)?;
        chan.write_to(&file)?;
        Ok(())
    } else {
        Err(IoErr::from(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot find feed at {}", path.to_string_lossy())
        )).into())
    }
}
