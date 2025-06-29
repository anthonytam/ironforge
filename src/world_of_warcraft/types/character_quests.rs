use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterQuestsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterQuestsCharacter,
    pub in_progress: Vec<CharacterQuest>,
    pub completed: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCompletedQuestsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterQuestsCharacter,
    pub quests: Vec<CharacterCompletedQuest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterQuestsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterQuestsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterQuestsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterQuest {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCompletedQuest {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub completed_timestamp: u64,
}
