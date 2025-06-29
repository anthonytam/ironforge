use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::covenant::*};

impl WorldOfWarcraftClient {
    pub async fn conduit(
        &self,
        conduit_id: u32,
    ) -> Result<ConduitResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/covenant/conduit/{conduit_id}"),
                "static",
            )
            .await
    }

    pub async fn conduit_index(&self) -> Result<ConduitIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/conduit/index".to_string(), "static")
            .await
    }

    pub async fn covenant(
        &self,
        covenant_id: u32,
    ) -> Result<CovenantResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/covenant/{covenant_id}"), "static")
            .await
    }

    pub async fn covenant_index(&self) -> Result<CovenantIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/index".to_string(), "static")
            .await
    }

    pub async fn covenant_media(
        &self,
        covenant_id: u32,
    ) -> Result<CovenantMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/covenant/{covenant_id}"),
                "static",
            )
            .await
    }

    pub async fn soulbind(
        &self,
        soulbind_id: u32,
    ) -> Result<SoulbindResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/covenant/soulbind/{soulbind_id}"),
                "static",
            )
            .await
    }

    pub async fn soulbind_index(&self) -> Result<SoulbindIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/soulbind/index".to_string(), "static")
            .await
    }
}
