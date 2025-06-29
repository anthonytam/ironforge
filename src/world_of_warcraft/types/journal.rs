use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub encounters: Vec<JournalEncounterSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub creatures: Vec<JournalEncounterCreature>,
    pub items: Vec<JournalEncounterItem>,
    pub sections: Vec<JournalEncounterSection>,
    pub instance: JournalEncounterInstance,
    pub category: JournalEncounterCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterCreature {
    pub name: String,
    pub id: u32,
    pub creature_display: Option<JournalEncounterCreatureDisplay>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterCreatureDisplay {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterItem {
    pub item: JournalEncounterItemInfo,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterItemInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSection {
    pub title: String,
    pub body_text: String,
    pub sections: Option<Vec<JournalEncounterSection>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterInstance {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterCategory {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSearchParameters {
    pub _page: Option<u32>,
    pub instance_name: Option<String>,
    pub locale: Option<String>,
    pub orderby: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSearchResponseItem {
    pub key: Href,
    pub data: JournalEncounterSearchData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSearchData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub instance: JournalEncounterInstance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalExpansionIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub tiers: Vec<JournalExpansionSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalExpansionSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalExpansionResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub dungeons: Vec<JournalExpansionDungeon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalExpansionDungeon {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub instances: Vec<JournalInstanceSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub map: JournalInstanceMap,
    pub area: JournalInstanceArea,
    pub description: String,
    pub encounters: Vec<JournalInstanceEncounter>,
    pub expansion: JournalInstanceExpansion,
    pub modes: Vec<JournalInstanceMode>,
    pub media: JournalInstanceMedia,
    pub minimum_level: u32,
    pub category: JournalInstanceCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceMap {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceArea {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceEncounter {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceExpansion {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceMode {
    pub mode: JournalInstanceModeInfo,
    pub players: u32,
    pub is_tracked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceModeInfo {
    #[serde(rename = "type")]
    pub mode_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceCategory {
    #[serde(rename = "type")]
    pub category_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceMediaResponse {
    pub assets: Vec<JournalInstanceMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalInstanceMediaAsset {
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
    pub key: String,
    pub value: String,
}
