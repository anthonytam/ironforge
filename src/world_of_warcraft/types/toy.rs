use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToyIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub toys: Vec<ToySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToySummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Toy {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub item: ToyItem,
    pub source: TypeNode,
    pub should_exclude_if_uncollected: bool,
    pub media: ToyMedia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToyItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToyMedia {
    pub key: Href,
    pub id: u32,
}
