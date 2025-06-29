use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEncountersSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterEncountersCharacter,
    pub dungeons: Href,
    pub raids: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterEncountersCharacter,
    pub expansions: Vec<CharacterDungeonExpansion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterEncountersCharacter,
    pub expansions: Vec<CharacterRaidExpansion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEncountersCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterEncountersRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEncountersRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonExpansion {
    pub expansion: CharacterDungeonExpansionInfo,
    pub instances: Vec<CharacterDungeonInstance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonExpansionInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonInstance {
    pub instance: CharacterDungeonInstanceInfo,
    pub modes: Vec<CharacterDungeonMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonInstanceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonMode {
    pub difficulty: CharacterDungeonDifficulty,
    pub status: CharacterDungeonStatus,
    pub progress: CharacterDungeonProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonDifficulty {
    #[serde(rename = "type")]
    pub difficulty_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonStatus {
    #[serde(rename = "type")]
    pub status_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonProgress {
    pub completed_count: u32,
    pub total_count: u32,
    pub encounters: Vec<CharacterDungeonEncounter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonEncounter {
    pub encounter: CharacterDungeonEncounterInfo,
    pub completed_count: u32,
    pub last_kill_timestamp: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDungeonEncounterInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidExpansion {
    pub expansion: CharacterRaidExpansionInfo,
    pub instances: Vec<CharacterRaidInstance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidExpansionInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidInstance {
    pub instance: CharacterRaidInstanceInfo,
    pub modes: Vec<CharacterRaidMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidInstanceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidMode {
    pub difficulty: CharacterRaidDifficulty,
    pub status: CharacterRaidStatus,
    pub progress: CharacterRaidProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidDifficulty {
    #[serde(rename = "type")]
    pub difficulty_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidStatus {
    #[serde(rename = "type")]
    pub status_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidProgress {
    pub completed_count: u32,
    pub total_count: u32,
    pub encounters: Vec<CharacterRaidEncounter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidEncounter {
    pub encounter: CharacterRaidEncounterInfo,
    pub completed_count: u32,
    pub last_kill_timestamp: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRaidEncounterInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
