use serde::{Deserialize, Serialize};

use super::common::Links;

#[derive(Debug, Serialize, Deserialize)]
pub struct WowTokenIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub last_updated_timestamp: u64,
    pub price: u64,
}
