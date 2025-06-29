use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub affixes: Vec<MythicKeystoneAffixSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffix {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub media: MythicKeystoneAffixMedia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<MythicKeystoneAffixMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneAffixMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}
