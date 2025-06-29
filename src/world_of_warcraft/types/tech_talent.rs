use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TechTalentIndexResponse {
    pub talents: Vec<TalentSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TechTalentMediaResponse {
    pub assets: Vec<MediaAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TechTalentResponse {
    pub key: Key,
    pub name: String,
    pub id: u32,
    #[serde(rename = "display_order")]
    pub display_order: u32,
    pub media: Media,
    #[serde(rename = "talent_tree")]
    pub talent_tree: Media,
    pub tier: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TechTalentTreeIndexResponse {
    #[serde(rename = "talent_trees")]
    pub talent_trees: Vec<TalentTree>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TechTalentTreeResponse {
    pub id: u32,
    #[serde(rename = "max_tiers")]
    pub max_tiers: u32,
    #[serde(rename = "playable_class")]
    pub playable_class: ClassSummary,
    pub talents: Vec<TalentSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub key: Key,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentTree {
    pub key: Key,
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentSummary {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}
