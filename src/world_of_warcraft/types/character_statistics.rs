use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterStatisticsCharacter,
    pub categories: Vec<CharacterStatisticsCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterStatisticsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsCategory {
    pub id: u32,
    pub name: String,
    pub sub_categories: Vec<CharacterStatisticsSubCategory>,
    pub statistics: Vec<CharacterStatisticsStatistic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsSubCategory {
    pub id: u32,
    pub name: String,
    pub statistics: Vec<CharacterStatisticsStatistic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatisticsStatistic {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub last_updated_timestamp: u64,
    pub money: Option<u64>,
}
