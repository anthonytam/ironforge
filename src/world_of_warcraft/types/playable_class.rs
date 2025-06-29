use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub gender_name: PlayableClassGenderName,
    pub power_type: PlayableClassPowerType,
    pub specializations: Vec<PlayableClassSpecialization>,
    pub media: Href,
    pub pvp_talent_slots: Href,
    pub playable_races: Vec<PlayableClassRace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character_classes: Vec<PlayableClassSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<PlayableClassMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpTalentSlotsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub talent_slots: Vec<PvpTalentSlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassGenderName {
    pub male: String,
    pub female: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassPowerType {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassRace {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClassMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpTalentSlot {
    pub slot_number: u32,
    pub unlock_player_level: u32,
}
