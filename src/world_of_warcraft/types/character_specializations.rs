use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecializationsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterSpecializationsCharacter,
    pub specializations: Vec<CharacterSpecialization>,
    pub active_specialization: CharacterActiveSpecialization,
    pub character_specializations: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecializationsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterSpecializationsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecializationsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecialization {
    pub specialization: CharacterSpecializationInfo,
    pub talents: Vec<CharacterTalent>,
    pub pvp_talent_slots: Vec<CharacterPvpTalentSlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecializationInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTalent {
    pub talent: CharacterTalentInfo,
    pub spell_tooltip: CharacterTalentSpellTooltip,
    pub column_index: u32,
    pub rank_index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTalentInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTalentSpellTooltip {
    pub spell: CharacterTalentSpellInfo,
    pub description: String,
    pub cast_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTalentSpellInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTalentSlot {
    pub selected: Option<CharacterPvpTalent>,
    pub slot_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTalent {
    pub talent: CharacterPvpTalentInfo,
    pub spell_tooltip: CharacterPvpTalentSpellTooltip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTalentInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTalentSpellTooltip {
    pub spell: CharacterPvpTalentSpellInfo,
    pub description: String,
    pub cast_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTalentSpellInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterActiveSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
