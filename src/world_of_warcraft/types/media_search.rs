use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MediaSearchParameters {
    pub _page: Option<u32>,
    pub orderby: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaSearchResponseItem {
    pub key: Key,
    pub data: MediaSearchData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaSearchData {
    pub assets: Vec<MediaAsset>,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaSearchResponse {
    pub results: Vec<MediaSearchResponseItem>,
    pub page: u32,
    #[serde(rename = "page_size")]
    pub page_size: u32,
    #[serde(rename = "max_page_size")]
    pub max_page_size: u32,
    #[serde(rename = "page_count")]
    pub page_count: u32,
    #[serde(rename = "result_count")]
    pub result_count: u32,
    #[serde(rename = "result_count_capped")]
    pub result_count_capped: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaAsset {
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
    pub key: String,
    pub value: String,
}
