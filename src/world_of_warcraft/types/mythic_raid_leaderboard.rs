use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidLeaderboardIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub current_leaderboards: Vec<MythicRaidLeaderboardSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidLeaderboardSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidLeaderboard {
    #[serde(rename = "_links")]
    pub links: Links,
    pub map: MythicRaidLeaderboardMap,
    pub period: u32,
    pub period_start_timestamp: u64,
    pub period_end_timestamp: u64,
    pub connected_realm: MythicRaidLeaderboardRealm,
    pub leading_groups: Vec<MythicRaidGroup>,
    pub map_challenge_mode_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidLeaderboardMap {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidLeaderboardRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidGroup {
    pub ranking: u32,
    pub duration: u64,
    pub completed_timestamp: u64,
    pub keystone_level: u32,
    pub members: Vec<MythicRaidGroupMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidGroupMember {
    pub profile: MythicRaidProfile,
    pub faction: TypeNode,
    pub specialization: MythicRaidSpecialization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidProfile {
    pub name: String,
    pub id: u32,
    pub realm: MythicRaidRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicRaidSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
