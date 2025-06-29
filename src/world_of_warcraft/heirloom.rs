use super::{
    WorldOfWarcraftClient,
    types::heirloom::{Heirloom, HeirloomIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_heirloom_index(&self) -> Result<HeirloomIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/heirloom/index".to_string(), "static")
            .await
    }

    pub async fn get_heirloom(&self, id: u32) -> Result<Heirloom, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/heirloom/{id}"), "static")
            .await
    }
}
