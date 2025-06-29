use super::{
    WorldOfWarcraftClient,
    types::reputations::{
        ReputationFaction, ReputationFactionIndex, ReputationTiers, ReputationTiersIndex,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_reputation_faction_index(
        &self,
    ) -> Result<ReputationFactionIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/reputation-faction/index".to_string(), "static")
            .await
    }
    pub async fn get_reputation_faction(
        &self,
        id: u32,
    ) -> Result<ReputationFaction, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/reputation-faction/{id}"), "static")
            .await
    }
    pub async fn get_reputation_tiers_index(
        &self,
    ) -> Result<ReputationTiersIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/reputation-tiers/index".to_string(), "static")
            .await
    }
    pub async fn get_reputation_tiers(
        &self,
        id: u32,
    ) -> Result<ReputationTiers, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/reputation-tiers/{id}"), "static")
            .await
    }
}
