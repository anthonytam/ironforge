use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMediaSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterMediaCharacter,
    pub assets: Vec<CharacterMediaAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMediaCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterMediaRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMediaRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}
