use super::common::{Href, Links};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationFactionIndexResponse {
    pub factions: Vec<FactionSummary>,
    #[serde(rename = "root_factions")]
    pub root_factions: Vec<FactionSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationFactionResponse {
    pub description: String,
    pub id: u32,
    pub name: String,
    #[serde(rename = "reputation_tiers")]
    pub reputation_tiers: ReputationTiersItem,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationTiersIndexResponse {
    #[serde(rename = "reputation_tiers")]
    pub reputation_tiers: Vec<ReputationTier>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationTiersResponse {
    pub faction: Option<FactionSummary>,
    pub id: u32,
    pub tiers: Vec<Tier>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationTier {
    pub key: Key,
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReputationTiersItem {
    pub key: Key,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tier {
    pub id: u32,
    #[serde(rename = "max_value")]
    pub max_value: u32,
    #[serde(rename = "min_value")]
    pub min_value: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FactionSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationFactionIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub reputation_factions: Vec<ReputationFactionSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationFactionSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationFaction {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub reputation_tiers: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationTiersIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub reputation_tiers: Vec<ReputationTiersSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationTiersSummary {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationTiers {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub tiers: Vec<ReputationTier>,
}
