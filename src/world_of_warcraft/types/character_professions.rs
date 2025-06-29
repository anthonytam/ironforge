use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterProfessionsCharacter,
    pub primaries: Vec<CharacterProfession>,
    pub secondaries: Vec<CharacterProfession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterProfessionsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfession {
    pub profession: CharacterProfessionInfo,
    pub tiers: Vec<CharacterProfessionTier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionTier {
    pub tier: CharacterProfessionTierInfo,
    pub skill_points: u32,
    pub max_skill_points: u32,
    pub known_recipes: Vec<CharacterProfessionRecipe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionTierInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfessionRecipe {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
