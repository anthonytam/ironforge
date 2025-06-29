use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub realms: Vec<RealmSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmRegion {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RealmSearchParameters {
    pub _page: Option<u32>,
    pub orderby: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmSearchResponse {
    pub results: Vec<RealmSummary>,
    pub page: u32,
    pub page_size: u32,
    pub max_page_size: u32,
    pub page_count: u32,
    pub result_count: u32,
    pub result_count_capped: bool,
}
