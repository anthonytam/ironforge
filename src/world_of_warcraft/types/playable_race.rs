use super::common::{Href, Links};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub gender_name: PlayableRaceGenderName,
    pub faction: PlayableRaceFaction,
    pub is_selectable: bool,
    pub is_allied_race: bool,
    pub playable_classes: Vec<PlayableRaceClass>,
    pub race: Option<PlayableRaceInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub races: Vec<PlayableRaceSummary>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceGenderName {
    pub male: String,
    pub female: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceFaction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableRaceSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
