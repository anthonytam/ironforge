use super::{
    WorldOfWarcraftClient,
    types::toy::{Toy, ToyIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_toy_index(&self) -> Result<ToyIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/toy/index".to_string(), "static")
            .await
    }

    pub async fn get_toy(&self, id: u32) -> Result<Toy, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/toy/{id}"), "static")
            .await
    }
}
