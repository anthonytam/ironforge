use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub emblems: Vec<GuildCrestEmblem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblem {
    pub id: u32,
    pub media: GuildCrestEmblemMediaSummary
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblemMediaSummary {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestBorderMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<GuildCrestBorderAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestBorderAsset {
    #[serde(rename = "_links")]
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblemMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<GuildCrestEmblemAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblemAsset {
    #[serde(rename = "_links")]
    pub key: String,
    pub value: String,
}