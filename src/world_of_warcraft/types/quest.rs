use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub title: String,
    pub area: QuestArea,
    pub description: String,
    pub requirements: QuestRequirements,
    pub rewards: Option<QuestRewards>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub quests: Vec<QuestSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestAreaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub area: String,
    pub quests: Vec<QuestSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestAreaIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub areas: Vec<QuestAreaSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestCategoryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub category: String,
    pub quests: Vec<QuestSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestCategoryIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub categories: Vec<QuestCategorySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestTypeResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    #[serde(rename = "type")]
    pub quest_type: String,
    pub quests: Vec<QuestSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestTypeIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub types: Vec<QuestTypeSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestArea {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestRequirements {
    pub min_character_level: Option<u32>,
    pub max_character_level: Option<u32>,
    pub faction: Option<QuestFaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestFaction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestRewards {
    pub experience: Option<u32>,
    pub money: Option<u32>,
    pub items: Option<Vec<QuestRewardItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestRewardItem {
    pub item: QuestItem,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestAreaSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestCategorySummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestTypeSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
