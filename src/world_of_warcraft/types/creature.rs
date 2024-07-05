use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub creature_families: Vec<CreatureFamilySummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilySummary {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamily {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String, 
    pub specialization: CreatureFamilySpecialization,
    pub media: CreatureFamilyMediaSummary
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilySpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyMediaSummary {
    pub key: Href,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureTypesIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub creature_types: Vec<CreatureTypeSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureTypeSummary {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureType {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub creature_type: CreatureType,
    pub family: CreatureFamilySummary,
    pub creature_displays: Vec<CreatureDisplay>,
    pub is_tameable: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureDisplay {
    pub key: Href,
    pub id: u32
}