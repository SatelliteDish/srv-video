use rss::Channel;
use thiserror::Error;

mod metadata;
pub use metadata::VideoMetadata;

mod source;
pub use source::VideoSource;

mod post;
pub use post::VideoPost;

const EXTENSION_PREFIX: &str = "srv";


#[derive(Debug,Clone,Error)]
pub enum FeedError {
    #[error("Missing required field \"{0}\"")]
    MissingRequiredField(String),
    #[error("\"{field}\" had an unexpected value of \"{value}\"")]
    InvalidField {
        field: String,
        value: String,
    },
}


#[derive(Debug, Clone)]
pub struct Feed {
    pub title: String,
    pub link: String,
    pub description: String,
    // None indicates stale value, if feed is fresh but without posts it'll be an empty Vec
    pub items: Option<Vec<VideoPost>>,
    pub image: Option<String>,
}

impl Default for Feed {
    fn default() -> Self {
        Self {
            title: String::default(),
            link: String::default(),
            description: String::default(),
            image: None,
            items: None,
        }
    }
}

impl From<Channel> for Feed {
    fn from(ch: Channel) -> Self {
        Self {
            title: ch.title.clone(),
            link: ch.link.clone(),
            description: ch.description.clone(),
            image: None,
            items: Some(ch.items().into_iter().map(|it| VideoPost::try_from(it.clone()).unwrap()).collect())
        }
    }
}

impl Feed {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
