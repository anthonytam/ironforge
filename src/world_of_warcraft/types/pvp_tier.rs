use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpTierIndexResponse {
    pub tiers: Vec<TierSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpTierResponse {
    pub key: Key,
    pub name: String,
    pub id: u32,
    pub bracket: Bracket,
    #[serde(rename = "max_rating")]
    pub max_rating: u32,
    pub media: Media,
    #[serde(rename = "min_rating")]
    pub min_rating: u32,
    #[serde(rename = "rating_type")]
    pub rating_type: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpTierMediaResponse {
    pub assets: Vec<MediaAsset>,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bracket {
    pub id: u32,
    #[serde(rename = "type")]
    pub bracket_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub key: Key,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TierSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaAsset {
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
    pub key: String,
    pub value: String,
}
