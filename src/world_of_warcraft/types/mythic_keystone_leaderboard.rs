use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneLeaderboardIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub current_leaderboards: Vec<MythicKeystoneLeaderboardSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneLeaderboardSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneLeaderboard {
    #[serde(rename = "_links")]
    pub links: Links,
    pub map: MythicKeystoneLeaderboardMap,
    pub period: u32,
    pub period_start_timestamp: u64,
    pub period_end_timestamp: u64,
    pub connected_realm: MythicKeystoneLeaderboardRealm,
    pub leading_groups: Vec<MythicKeystoneGroup>,
    pub keystone_affixes: Vec<MythicKeystoneAffixReference>,
    pub map_challenge_mode_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneLeaderboardMap {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneLeaderboardRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneGroup {
    pub ranking: u32,
    pub duration: u64,
    pub completed_timestamp: u64,
    pub keystone_level: u32,
    pub members: Vec<MythicKeystoneGroupMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneGroupMember {
    pub profile: MythicKeystoneProfile,
    pub faction: TypeNode,
    pub specialization: MythicKeystoneSpecialization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneProfile {
    pub name: String,
    pub id: u32,
    pub realm: MythicKeystoneRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixReference {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
