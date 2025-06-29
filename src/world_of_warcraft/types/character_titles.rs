use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitlesSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterTitlesCharacter,
    pub active_title: Option<CharacterTitle>,
    pub titles: Vec<CharacterTitle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitlesCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterTitlesRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitlesRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitle {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub display_string: String,
}
