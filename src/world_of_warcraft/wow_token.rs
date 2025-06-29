use super::{WorldOfWarcraftClient, types::wow_token::WowTokenIndex};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_wow_token_index(&self) -> Result<WowTokenIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/token/index".to_string(), "dynamic")
            .await
    }
}
