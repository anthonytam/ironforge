use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpSeasonIndexResponse {
    pub current_season: Season,
    pub seasons: Vec<Season>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpSeasonResponse {
    pub id: u32,
    pub leaderboards: LeaderboardLink,
    pub rewards: RewardLink,
    #[serde(rename = "season_name")]
    pub season_name: Option<String>,
    #[serde(rename = "season_start_timestamp")]
    pub season_start_timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpLeaderboardIndexResponse {
    pub leaderboards: Vec<LeaderboardSummary>,
    pub season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpLeaderboardResponse {
    pub bracket: Bracket,
    pub entries: Vec<Entry>,
    pub name: String,
    pub season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpRewardsIndexResponse {
    pub rewards: Vec<Reward>,
    pub season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bracket {
    pub id: u32,
    #[serde(rename = "type")]
    pub bracket_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub key: Key,
    pub name: String,
    pub id: u32,
    pub realm: Realm,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub character: Character,
    pub faction: FactionType,
    pub rank: u32,
    pub rating: u32,
    #[serde(rename = "season_match_statistics")]
    pub season_match_statistics: SeasonMatchStatistics,
    pub tier: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Realm {
    pub key: Key,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reward {
    pub achievement: AchievementSummary,
    pub bracket: Bracket,
    pub faction: Option<Faction>,
    #[serde(rename = "rating_cutoff")]
    pub rating_cutoff: u32,
    pub specialization: Option<SpecializationSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub key: Key,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonMatchStatistics {
    pub lost: u32,
    pub played: u32,
    pub won: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LeaderboardLink {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewardLink {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FactionType {
    #[serde(rename = "type")]
    pub faction_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LeaderboardSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AchievementSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Faction {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecializationSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}
