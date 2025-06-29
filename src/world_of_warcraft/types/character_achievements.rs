use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub total_quantity: u32,
    pub total_points: u32,
    pub achievements: Vec<CharacterAchievement>,
    pub category_progress: Vec<CharacterAchievementCategoryProgress>,
    pub recent_events: Vec<CharacterAchievementRecentEvent>,
    pub character: CharacterAchievementCharacter,
    pub statistics: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementStatisticsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterAchievementCharacter,
    pub categories: Vec<CharacterAchievementStatisticsCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievement {
    pub id: u32,
    pub achievement: CharacterAchievementInfo,
    pub criteria: Option<CharacterAchievementCriteria>,
    pub completed_timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementCriteria {
    pub id: u32,
    pub amount: u32,
    pub is_completed: bool,
    pub child_criteria: Option<Vec<CharacterAchievementCriteria>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementCategoryProgress {
    pub category: CharacterAchievementCategory,
    pub quantity: u32,
    pub points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementCategory {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementRecentEvent {
    pub achievement: CharacterAchievementInfo,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterAchievementRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementStatisticsCategory {
    pub id: u32,
    pub name: String,
    pub sub_categories: Vec<CharacterAchievementStatisticsSubCategory>,
    pub statistics: Vec<CharacterAchievementStatistic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementStatisticsSubCategory {
    pub id: u32,
    pub name: String,
    pub statistics: Vec<CharacterAchievementStatistic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAchievementStatistic {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub last_updated_timestamp: u64,
    pub money: Option<u64>,
}
