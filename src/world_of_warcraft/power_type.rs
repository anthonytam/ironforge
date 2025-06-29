use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::power_type::{PowerTypeIndexResponse, PowerTypeResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_power_type(
        &self,
        power_type_id: u32,
    ) -> Result<PowerTypeResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/power-type/{power_type_id}"), "static")
            .await
    }

    pub async fn get_power_type_index(
        &self,
    ) -> Result<PowerTypeIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/power-type/index".to_string(), "static")
            .await
    }
}
