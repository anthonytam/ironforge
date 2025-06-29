use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayableRaceResponse {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayableRaceIndexResponse {
    pub races: Vec<RaceSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RaceSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

impl WorldOfWarcraftClient {
    pub async fn get_playable_race(
        &self,
        race_id: u32,
    ) -> Result<PlayableRaceResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/playable-race/{race_id}"), "static")
            .await
    }
    pub async fn get_playable_race_index(
        &self,
    ) -> Result<PlayableRaceIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/playable-race/index".to_string(), "static")
            .await
    }
}
