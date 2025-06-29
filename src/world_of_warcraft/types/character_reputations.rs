use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterReputationsCharacter,
    pub reputations: Vec<CharacterReputation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterReputationsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputation {
    pub faction: CharacterReputationFaction,
    pub standing: CharacterReputationStanding,
    pub paragon: Option<CharacterReputationParagon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationFaction {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationStanding {
    pub raw: i32,
    pub value: i32,
    pub max: i32,
    pub tier: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationParagon {
    pub raw: u32,
    pub value: u32,
    pub max: u32,
    pub threshold: u32,
}
