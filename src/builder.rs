use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use quick_xml::{DeError, se::to_string};

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename = "urlset")]
pub(crate) struct UrlSet {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:xhtml")]
    pub xmlns_xhtml: String,
    pub url: Vec<Url>
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Url {
    pub loc: String,
    pub priority: Option<String>,
}

impl UrlSet {
    pub fn new(urls: Vec<String>) -> Self {
        // xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml"  
        UrlSet {
            xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9".to_string(),
            xmlns_xhtml:"http://www.w3.org/1999/xhtml".to_string(),
            url: urls
                .into_iter()
                .map(|url| Url {
                    loc: url.replace(".md",".html"),
                    priority: Some("1.0".to_string()),
                })
                .collect(),
        }
    }

    pub fn to_xml(&self) -> Result<String,DeError> {
        to_string(&self)
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = if self.priority.is_some() { 2 } else { 1 };
        let mut map = serializer.serialize_map(Some(len))?;

        map.serialize_entry("loc", &self.loc)?;
        if let Some(priority) = &self.priority {
            map.serialize_entry("priority", priority)?;
        }

        map.end()
    }
}
