use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub emblems: Vec<GuildCrestEmblem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblem {
    pub id: u32,
    pub media: GuildCrestEmblemMediaSummary,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestBorderEmblemResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<GuildCrestAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestComponentsIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub emblems: Vec<GuildCrestComponent>,
    pub borders: Vec<GuildCrestComponent>,
    pub colors: GuildCrestColors,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestComponent {
    pub id: u32,
    pub media: GuildCrestMedia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestColors {
    pub backgrounds: Vec<GuildCrestBackground>,
    pub borders: Vec<GuildCrestBackground>,
    pub emblems: Vec<GuildCrestBackground>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestBackground {
    pub id: u32,
    pub rgba: GuildCrestRGBA,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestAsset {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}
