use super::{
    WorldOfWarcraftClient,
    types::region::{Region, RegionIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_region_index(&self) -> Result<RegionIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/region/index".to_string(), "dynamic")
            .await
    }

    pub async fn get_region(&self, id: u32) -> Result<Region, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/region/{id}"), "dynamic")
            .await
    }
}
