use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auctions {
    #[serde(rename = "_links")]
    pub links: Links,
    pub connected_realm: Href,
    pub auctions: Vec<Auction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityAuctions {
    #[serde(rename = "_links")]
    pub links: Links,
    pub auctions: Vec<CommodityAuction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auction {
    pub id: u64,
    pub item: AuctionItem,
    pub bid: Option<u64>,
    pub buyout: u64,
    pub quantity: u64,
    pub time_left: String // See TODO below.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityAuction {
    pub id: u64,
    pub item: CommodityAuctionItem,
    pub quantity: u64,
    pub unit_price: u64,
    pub time_left: String // See TODO below.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityAuctionItem {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionItem {
    pub id: u64,
    pub context: Option<u32>,
    pub bonus_lists: Option<Vec<u32>>,
    pub modifiers: Vec<AuctionItemModifier>,
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

// TODO: Come back to this
// pub enum AuctionTimeLeft {
//     SHORT = "SHORT",
//     LONG = "LONG",
//     VERY_LONG = "VERY_LONG"
// }

