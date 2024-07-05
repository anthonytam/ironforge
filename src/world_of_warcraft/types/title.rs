use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub titles: Vec<TitleSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleSummary {
    pub key: Href,
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub gender_name: TitleGenderName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleGenderName {
    pub male: String,
    pub female: String,
}