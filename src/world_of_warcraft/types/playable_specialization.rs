use super::common::{Href, Links};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub gender_description: PlayableSpecializationGenderDescription,
    pub playable_class: PlayableSpecializationClass,
    pub media: Href,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub specializations: Vec<PlayableSpecializationSummary>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<PlayableSpecializationMediaAsset>,
    pub id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationGenderDescription {
    pub male: String,
    pub female: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableSpecializationMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}
