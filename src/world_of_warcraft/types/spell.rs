use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<SpellMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchParameters {
    pub _page: Option<u32>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub orderby: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchResponseItem {
    pub key: Href,
    pub data: SpellSearchData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchData {
    pub id: u32,
    pub media: SpellSearchMedia,
    pub name: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchMedia {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub page: u32,
    pub page_size: u32,
    pub max_page_size: u32,
    pub page_count: u32,
    pub results: Vec<SpellSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSearchResult {
    pub key: Href,
    pub data: SpellSearchData,
}
