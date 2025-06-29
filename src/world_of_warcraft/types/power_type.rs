use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerTypeIndexResponse {
    pub power_types: Vec<PowerTypeSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerTypeResponse {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerTypeSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}
