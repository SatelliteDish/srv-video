use rss::{
    Item as RssItem,
    extension::Extension,
};
use std::collections::{
    BTreeMap,
};

use super::{
    VideoMetadata,
    VideoSource,
    EXTENSION_PREFIX,
    FeedError,
};

#[derive(Debug,Clone)]
pub struct VideoPost {
    pub meta: VideoMetadata,
    pub comment_url: Option<String>,
    pub rating_url: Option<String>,
    pub src_set: Vec<VideoSource>,
}

impl From<VideoPost> for BTreeMap<String, Vec<Extension>> {
    fn from(value: VideoPost) -> Self {
        let VideoPost { meta, comment_url, rating_url, src_set } = value;
        let mut out: Self = BTreeMap::new();
        out.insert(
            format!("{EXTENSION_PREFIX}:video"),
            vec![meta.into()],
        );
        if let Some(url) = comment_url {
            let name = format!("{EXTENSION_PREFIX}:comments");
            out.insert(
                name.clone(),
                vec![Extension {
                    name,
                    value: None,
                    attrs: [("url".to_string(),url)].into(),
                    children: BTreeMap::new(),
                }],
            );
        }
        if let Some(url) = rating_url {
            let name = format!("{EXTENSION_PREFIX}:ratings");
            out.insert(
                name.clone(),
                vec![Extension {
                    name,
                    value: None,
                    attrs: [("url".to_string(),url)].into(),
                    children: BTreeMap::new(),
                }],
            );
        }
        out.insert(
            format!("{EXTENSION_PREFIX}:sources"),
            vec![Extension {
                name: format!("{EXTENSION_PREFIX}:sources"),
                value: None,
                attrs: BTreeMap::new(),
                children: BTreeMap::from([
                    (
                        "source".to_string(),
                        src_set.into_iter().map(VideoSource::into).collect(),
                    )
                ]),
            }],
        );

        out
    }
}


impl From<VideoPost> for RssItem {
    fn from(value: VideoPost) -> Self {
        Self {
            title: Some(value.meta.title.clone()),
            description: value.meta.description.clone(),
            extensions: BTreeMap::from([(format!("{EXTENSION_PREFIX}"),value.into())]),
            ..Default::default()
        }
    }
}

impl TryFrom<RssItem> for VideoPost {
    type Error = FeedError;
    fn try_from(value: RssItem) -> Result<Self, Self::Error> {
        let ext = value.extensions.get("srv").ok_or(FeedError::MissingRequiredField("srv".to_string()))?;
        let comment_url: Option<String> = ext.get(&format!("comments"))
            .map(|cm_ext| {
                cm_ext.get(0)
                    .map(|e| e.value.clone())
                    .ok_or(FeedError::MissingRequiredField(format!("{EXTENSION_PREFIX}:comments")))
            }).into_iter().collect::<Result<Option<_>,_>>()?;
        let rating_url: Option<String> = ext.get(&format!("ratings"))
            .map(|cm_ext| {
                cm_ext.get(0)
                    .map(|e| e.value.clone())
                    .ok_or(FeedError::MissingRequiredField(format!("{EXTENSION_PREFIX}:ratings")))
            }).into_iter().collect::<Result<Option<_>,_>>()?;

        Ok(VideoPost {
            meta: ext.get("video").unwrap().get(0).unwrap().clone().try_into().unwrap(),
            comment_url,
            rating_url,
            src_set: ext.get("sources")
                .ok_or(FeedError::MissingRequiredField(format!("{EXTENSION_PREFIX}:sources")))?
                .into_iter()
                .flat_map(|sources| sources.children.get("source").into_iter().flatten())
                .map(|src| VideoSource::try_from(src.attrs.clone()).unwrap())
                .collect(),
        })
    }
}

