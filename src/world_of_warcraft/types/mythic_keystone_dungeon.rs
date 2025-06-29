use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub dungeons: Vec<MythicKeystoneDungeonSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeon {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub map: MythicKeystoneDungeonMap,
    pub zone: MythicKeystoneDungeonZone,
    pub dungeon: MythicKeystoneDungeonReference,
    pub keystone_upgrades: Vec<MythicKeystoneUpgrade>,
    pub media: MythicKeystoneDungeonMedia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonMap {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonZone {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonReference {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneUpgrade {
    pub upgrade_level: u32,
    pub qualifying_duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneDungeonMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub dungeons: Href,
    pub seasons: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystonePeriodIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub current_period: MythicKeystonePeriodSummary,
    pub periods: Vec<MythicKeystonePeriodSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystonePeriodSummary {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystonePeriod {
    #[serde(rename = "_links")]
    pub links: Links,
    pub end_timestamp: u64,
    pub id: u32,
    pub start_timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneSeasonIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub current_season: MythicKeystoneSeasonSummary,
    pub seasons: Vec<MythicKeystoneSeasonSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneSeasonSummary {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneSeason {
    #[serde(rename = "_links")]
    pub links: Links,
    pub end_timestamp: u64,
    pub id: u32,
    pub periods: Vec<MythicKeystonePeriodSummary>,
    pub season_name: Option<String>,
    pub start_timestamp: u64,
}
