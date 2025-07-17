use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub creature_families: Vec<CreatureFamilySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilySummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamily {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub specialization: CreatureFamilySpecialization,
    pub media: CreatureFamilyMediaSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilySpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyMediaSummary {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<CreatureFamilyMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureFamilyMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureTypesIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub creature_types: Vec<CreatureTypeSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureTypeSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureType {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
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
    pub is_tameable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureDisplay {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchParameters {
    pub _page: Option<u32>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub orderby: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchResponseItem {
    pub key: Href,
    pub data: CreatureSearchData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchData {
    pub creature_displays: Vec<CreatureSearchDisplay>,
    pub family: Option<CreatureSearchFamily>,
    pub id: u32,
    pub is_tameable: bool,
    pub name: Option<std::collections::HashMap<String, String>>,
    pub r#type: Option<CreatureSearchType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchDisplay {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchFamily {
    pub id: u32,
    pub name: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchType {
    pub id: u32,
    pub name: std::collections::HashMap<String, String>,
}
