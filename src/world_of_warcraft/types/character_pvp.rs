use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterPvpCharacter,
    pub brackets: Vec<CharacterPvpBracket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpBracketStatisticsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterPvpCharacter,
    pub bracket: CharacterPvpBracket,
    pub season: CharacterPvpSeason,
    pub tier: CharacterPvpTier,
    pub rating: u32,
    pub season_match_statistics: CharacterPvpSeasonMatchStatistics,
    pub weekly_match_statistics: CharacterPvpWeeklyMatchStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterPvpRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpBracket {
    pub id: u32,
    #[serde(rename = "type")]
    pub bracket_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpSeason {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpTier {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpSeasonMatchStatistics {
    pub played: u32,
    pub won: u32,
    pub lost: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPvpWeeklyMatchStatistics {
    pub played: u32,
    pub won: u32,
    pub lost: u32,
}
