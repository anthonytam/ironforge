use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayableSpecializationResponse {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayableSpecializationIndexResponse {
    pub character_specializations: Vec<SpecializationSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayableSpecializationMediaResponse {
    pub assets: Vec<MediaAsset>,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecializationSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaAsset {
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
    pub key: String,
    pub value: String,
}

impl WorldOfWarcraftClient {
    pub async fn get_playable_specialization(
        &self,
        spec_id: u32,
    ) -> Result<PlayableSpecializationResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/playable-specialization/{spec_id}"),
                "static",
            )
            .await
    }
    pub async fn get_playable_specialization_index(
        &self,
    ) -> Result<PlayableSpecializationIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/playable-specialization/index".to_string(),
                "static",
            )
            .await
    }
    pub async fn get_playable_specialization_media(
        &self,
        spec_id: u32,
    ) -> Result<PlayableSpecializationMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/playable-specialization/{spec_id}"),
                "static",
            )
            .await
    }
}
