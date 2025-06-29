use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategoryIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub categories: Vec<AchievementCategoryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategoryEntry {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSummaryCategory {
    #[serde(rename = "_links")]
    pub links: Links,
    pub achievements: Vec<AchievementSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub achievements: Vec<AchievementSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub category: AchievementCategory,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub is_account_wide: bool,
    pub criteria: AchievementCriteria,
    pub next_achievement: Option<AchievementNextAchievement>,
    pub media: AchievementMediaReference,
    pub display_order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategory {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCriteria {
    pub id: u32,
    pub description: String,
    pub amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementNextAchievement {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementMediaReference {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<AchievementAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}
