use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub regions: Vec<Href>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub tag: String,
    pub patch_string: String,
}
