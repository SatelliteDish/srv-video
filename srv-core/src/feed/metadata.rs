use rss::extension::Extension;
use std::collections::{
    BTreeMap,
    HashMap,
};
use crate::feed::FeedError;

use super::EXTENSION_PREFIX;

#[derive(Debug,Clone)]
pub struct VideoMetadata {
    pub title: String,
    pub thumbnail: Option<String>, // URL to the thumbnail image
    pub description: Option<String>, // Video description
    pub categories: Vec<String>, // Categories the video belongs to
    // Links to things mentioned in the video. Could be stuff like sponsor links or it could be
    // things like a list of sources, or attribution for clips used
    pub references: HashMap<String,Vec<String>>,
    pub related: Vec<String>, // Used to populate suggested videos
}

impl From<VideoMetadata> for BTreeMap<String,Vec<Extension>> {
    fn from(value: VideoMetadata) -> Self {
        let mut out = BTreeMap::from([
            (
                format!("{EXTENSION_PREFIX}:title"),
                vec![Extension{
                    name: format!("{EXTENSION_PREFIX}:title"),
                    value: Some(value.title),
                    attrs: BTreeMap::new(),
                    children: BTreeMap::new(),
                }],
            ),
            (
                format!("{EXTENSION_PREFIX}:category"),
                value.categories.into_iter().map(|cat| Extension {
                    name: format!("{EXTENSION_PREFIX}:category"),
                    value: Some(cat),
                    attrs: BTreeMap::new(),
                    children: BTreeMap::new(),
                }).collect(),
            ),
            (
                format!("{EXTENSION_PREFIX}:related"),
                value.related.into_iter().map(|rel| Extension {
                    name: format!("{EXTENSION_PREFIX}:related"),
                    value: Some(rel),
                    attrs: BTreeMap::new(),
                    children: BTreeMap::new(),
                }).collect(),
            ),
        ]);
        if let Some(thumb) = value.thumbnail {
            out.insert(
                format!("{EXTENSION_PREFIX}:thumbnail"),
                vec![Extension{
                    name: format!("{EXTENSION_PREFIX}:thumbnail"),
                    value: Some(thumb),
                    attrs: BTreeMap::new(),
                    children: BTreeMap::new(),
                }],
            );
        }
        if let Some(desc) = value.description {
            out.insert(
                format!("{EXTENSION_PREFIX}:description"),
                vec![Extension{
                    name: format!("{EXTENSION_PREFIX}:description"),
                    value: Some(desc),
                    attrs: BTreeMap::new(),
                    children: BTreeMap::new(),
                }],
            );
        }

        let references = value.references.into_iter()
            .map(|(title, v)| v.into_iter().map(move|url| (title.clone(), url)))
            .flatten();
        out.insert(format!("{EXTENSION_PREFIX}:reference"), references.map(|(title,url)| Extension {
            name: format!("{EXTENSION_PREFIX}:reference"),
            value: Some(url),
            attrs: BTreeMap::from([("title".to_string(), title)]),
            children: BTreeMap::new(),
        }).collect());

        out
    }
}

impl From<VideoMetadata> for Extension {
    fn from(value: VideoMetadata) -> Self {
        Self {
            name: format!("{EXTENSION_PREFIX}:video"),
            value: None,
            attrs: BTreeMap::new(),
            children: value.into(),
        }
    }
}

impl TryFrom<Extension> for VideoMetadata {
    type Error = FeedError;
    fn try_from(value: Extension) -> Result<Self, Self::Error> {
        let chld = value.children();
        let title: String = chld.get("title").map(|ttl| {
            ttl.get(0)
                .map(|t| t.value.clone().ok_or(FeedError::InvalidField { field: format!("title"), value: format!("None") }))
                .ok_or(FeedError::MissingRequiredField(format!("title")))
        }).ok_or(FeedError::MissingRequiredField(format!("title")))???;
        let thumbnail: Option<String> = if let Some(desc) = chld.get("thumbnail") {
            desc.get(0)
                .map(|d| d.value.clone().ok_or(FeedError::InvalidField {
                    field: "thumbnail".to_string(),
                    value: "None".to_string(),
                }))
                .transpose()?
        } else { None };
        let description: Option<String> = if let Some(desc) = chld.get("description") {
            desc.get(0)
                .map(|d| d.value.clone().ok_or(FeedError::InvalidField {
                    field: "description".to_string(),
                    value: "None".to_string(),
                }))
                .transpose()?
        } else { None };
        let categories: Vec<String> = chld.get("category").unwrap_or(&vec![])
            .into_iter().map(|ext| ext.value.clone().ok_or(FeedError::InvalidField { field: format!("category"), value: format!("None") }))
            .collect::<Result<_,_>>()?;
        let related: Vec<String> = chld.get("related").unwrap_or(&vec![])
            .into_iter().map(|ext| ext.value.clone().ok_or(FeedError::InvalidField { field: format!("related"), value: format!("None") }))
            .collect::<Result<_,_>>()?;
        let references: HashMap<String, Vec<String>> = chld.get("reference").unwrap_or(&vec![])
            .into_iter()
            .map(|ext| {
                let key = ext.attrs.get("title")
                    .ok_or(FeedError::MissingRequiredField("reference[title]".to_string()))?;
                let val = ext.value.clone()
                    .ok_or(FeedError::InvalidField {
                        field: "reference".to_string(),
                        value: "None".to_string(),
                    })?;
                Ok((key.clone(), val))
            })
            .collect::<Result<Vec<(String, String)>, FeedError>>()?
            .into_iter()
            .fold(HashMap::new(), |mut map, (k, v)| {
                map.entry(k).or_insert_with(Vec::new).push(v);
                map
            });


        Ok(VideoMetadata {
            title,
            thumbnail,
            description,
            categories,
            related,
            references,
        })
    }
}
