use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct PetResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub battle_pet_type: PetBattlePetType,
    pub description: String,
    pub is_capturable: bool,
    pub is_tradable: bool,
    pub is_battlepet: bool,
    pub is_alliance_only: bool,
    pub is_horde_only: bool,
    pub abilities: Vec<PetAbility>,
    pub source: PetSource,
    pub icon: String,
    pub creature: Option<PetCreature>,
    pub is_random_creature_display: bool,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub pets: Vec<PetSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<PetMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbilityResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub battle_pet_type: PetBattlePetType,
    pub rounds: u32,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbilityIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub abilities: Vec<PetAbilitySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbilityMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<PetMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetBattlePetType {
    #[serde(rename = "type")]
    pub battle_pet_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbility {
    pub ability: PetAbilityInfo,
    pub slot: u32,
    pub required_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbilityInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetCreature {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetAbilitySummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
