use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct MountIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub mounts: Vec<MountSummary>,
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
    pub requirements: MountRequirements,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchParameters {
    pub _page: Option<u32>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub orderby: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchResponseItem {
    pub key: Href,
    pub data: MountSearchData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchData {
    pub creature_displays: Vec<MountSearchCreatureDisplay>,
    pub faction: Option<MountSearchFaction>,
    pub id: u32,
    pub name: Option<std::collections::HashMap<String, String>>,
    pub source: MountSearchSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchCreatureDisplay {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchFaction {
    pub name: std::collections::HashMap<String, String>,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchSource {
    pub name: std::collections::HashMap<String, String>,
    pub r#type: String,
}
