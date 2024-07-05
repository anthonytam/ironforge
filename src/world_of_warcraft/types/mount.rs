use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct MountIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub mounts: Vec<MountSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub creature_displays: Vec<MountCreatureDisplay>,
    pub description: String,
    pub source: TypeNode,
    pub faction: TypeNode,
    pub requirements: MountRequirements
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountCreatureDisplay {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountRequirements {
    pub faction: Option<TypeNode>,
}