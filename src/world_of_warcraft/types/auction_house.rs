use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auctions {
    #[serde(rename = "_links")]
    pub links: Links,
    pub connected_realm: Option<Href>,
    pub auctions: Vec<Auction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auction {
    pub id: u64,
    pub item: AuctionItem,
    pub bid: Option<u64>,
    pub buyout: Option<u64>,
    pub quantity: u64,
    pub unit_price: Option<u64>,
    pub time_left: AuctionTimeLeft
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum AuctionTimeLeft {
    SHORT,
    MEDIUM,
    LONG,
    VERY_LONG,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionItem {
    pub id: u64,
    pub context: Option<u32>,
    pub bonus_lists: Option<Vec<u32>>,
    pub modifiers: Option<Vec<AuctionItemModifier>>,
    pub pet_breed_id: Option<u32>,
    pub pet_level: Option<u32>,
    pub pet_quality_id: Option<u32>,
    pub pet_species_id: Option<u32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionItemModifier {
    #[serde(rename = "type")]
    pub modifier_type: u32,
    #[serde(rename = "value")]
    pub modifier_value: u32
}
