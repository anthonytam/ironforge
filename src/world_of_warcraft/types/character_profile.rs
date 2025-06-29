use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfileResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub gender: CharacterGender,
    pub faction: CharacterFaction,
    pub race: CharacterRace,
    pub character_class: CharacterClass,
    pub active_spec: CharacterSpecialization,
    pub realm: CharacterRealm,
    pub guild: Option<CharacterGuild>,
    pub level: u32,
    pub experience: u32,
    pub achievement_points: u32,
    pub last_login_timestamp: u64,
    pub average_item_level: u32,
    pub equipped_item_level: u32,
    pub active_title: Option<CharacterTitle>,
    pub covenants: Option<Vec<CharacterCovenant>>,
    pub character_appearance: Href,
    pub character_achievements: Href,
    pub character_collections: Href,
    pub character_encounters: Href,
    pub character_equipment: Href,
    pub character_hunter_pets: Href,
    pub character_media: Href,
    pub character_mythic_keystone_profile: Href,
    pub character_professions: Href,
    pub character_pvp_summary: Href,
    pub character_quests: Href,
    pub character_reputations: Href,
    pub character_soulbinds: Href,
    pub character_specializations: Href,
    pub character_statistics: Href,
    pub character_titles: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfileStatusResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub is_valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterGender {
    #[serde(rename = "type")]
    pub gender_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterFaction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRace {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterGuild {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterRealm,
    pub faction: CharacterFaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTitle {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCovenant {
    pub covenant: CharacterCovenantInfo,
    pub renown_level: u32,
    pub soulbinds: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCovenantInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
