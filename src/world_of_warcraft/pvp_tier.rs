use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::pvp_tier::{PvpTierIndexResponse, PvpTierMediaResponse, PvpTierResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_pvp_tier(
        &self,
        pvp_tier_id: u32,
    ) -> Result<PvpTierResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/pvp-tier/{pvp_tier_id}"), "static")
            .await
    }

    pub async fn get_pvp_tier_index(&self) -> Result<PvpTierIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/pvp-tier/index".to_string(), "static")
            .await
    }

    pub async fn get_pvp_tier_media(
        &self,
        pvp_tier_id: u32,
    ) -> Result<PvpTierMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/pvp-tier/{pvp_tier_id}"),
                "static",
            )
            .await
    }
}
