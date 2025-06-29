use super::{
    WorldOfWarcraftClient,
    types::title::{Title, TitleIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_title_index(&self) -> Result<TitleIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/title/index".to_string(), "static")
            .await
    }

    pub async fn get_title(&self, id: u32) -> Result<Title, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/title/{id}"), "static")
            .await
    }
}
