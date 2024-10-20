use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCateogoryIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub categories: Vec<AchievementCategoryEntry>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategoryEntry {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchivementSummaryCategory {
    #[serde(rename = "_links")]
    pub links: Links,
    pub achievements: Vec<AchievementSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub achievements: Vec<AchievementSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSummary {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub category: AchivementCategory,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub is_account_wide: bool,
    pub criteria: AchivementCriteria,
    pub next_achievement: Option<AchivementNextAchievement>,
    pub media: AchivementMedia,
    pub display_order: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchivementCategory {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchivementCriteria {
    pub id: u32,
    pub description: String,
    pub amount: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchivementNextAchievement {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchivementMedia {
    pub key: Href,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<AchievementAsset>,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32
}