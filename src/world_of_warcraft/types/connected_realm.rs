use serde::{Deserialize, Serialize};

use super::{common::{Href, Links, TypeNode}, realm::Realm};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedRealmsIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub connected_realms: Vec<Href>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectedRealm {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub has_queue: bool,
    pub status: TypeNode,
    pub population: TypeNode,
    pub realms: Vec<Realm>,
    pub mythic_leaderboards: Href,
    pub auctions: Href
}