use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub realm: GuildRealm,
    pub faction: GuildFaction,
    pub members: Vec<GuildMember>,
    pub achievements: GuildAchievements,
    pub created_timestamp: u64,
    pub activity: Option<GuildActivity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildFaction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMember {
    pub character: GuildCharacter,
    pub rank: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCharacter {
    pub name: String,
    pub id: u32,
    pub realm: GuildCharacterRealm,
    pub level: u32,
    pub playable_class: GuildPlayableClass,
    pub playable_race: GuildPlayableRace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCharacterRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildPlayableClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildPlayableRace {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievements {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivity {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievementsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub guild: GuildAchievementGuild,
    pub total_quantity: u32,
    pub total_points: u32,
    pub achievements: Vec<GuildAchievement>,
    pub category_progress: Vec<GuildCategoryProgress>,
    pub recent_activity: Vec<GuildRecentActivity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievementGuild {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: GuildRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievement {
    pub id: u32,
    pub achievement: GuildAchievementInfo,
    pub criteria: Option<GuildAchievementCriteria>,
    pub completed_timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievementInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildAchievementCriteria {
    pub id: u32,
    pub amount: u32,
    pub is_completed: bool,
    pub child_criteria: Option<Vec<GuildAchievementCriteria>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCategoryProgress {
    pub category: GuildCategory,
    pub quantity: u32,
    pub points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCategory {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildRecentActivity {
    pub character: GuildActivityCharacter,
    pub achievement: GuildAchievementInfo,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivityCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: GuildCharacterRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivityResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub guild: GuildAchievementGuild,
    pub activities: Vec<GuildActivityItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivityItem {
    pub character: GuildActivityCharacter,
    pub activity: GuildActivityInfo,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivityInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildRosterResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub guild: GuildAchievementGuild,
    pub members: Vec<GuildRosterMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildRosterMember {
    pub character: GuildCharacter,
    pub rank: u32,
    pub joined_timestamp: u64,
}
