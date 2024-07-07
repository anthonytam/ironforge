use serde::{Deserialize, Serialize};

use super::common::{TypeNode, Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub realms: Vec<RealmSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    pub id: u32,
    pub region: RealmRegion,
    pub connected_realm: Href,
    pub name: String,
    pub category: String,
    pub locale: String,
    pub timezone: String,
    #[serde(rename = "type")]
    pub realm_type: TypeNode,
    pub is_tournament: bool,
    pub slug: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmRegion {
    pub key: Href,
    pub name: String,
    pub id: u32,
}