use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::talent::{
        PvpTalentIndexResponse, PvpTalentResponse, TalentIndexResponse, TalentResponse,
        TalentTreeIndexResponse, TalentTreeNodesResponse, TalentTreeResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_pvp_talent(
        &self,
        pvp_talent_id: u32,
    ) -> Result<PvpTalentResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/pvp-talent/{pvp_talent_id}"), "static")
            .await
    }

    pub async fn get_pvp_talent_index(
        &self,
    ) -> Result<PvpTalentIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/pvp-talent/index".to_string(), "static")
            .await
    }

    pub async fn get_talent(
        &self,
        talent_id: u32,
    ) -> Result<TalentResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/talent/{talent_id}"), "static")
            .await
    }

    pub async fn get_talent_index(&self) -> Result<TalentIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/talent/index".to_string(), "static")
            .await
    }

    pub async fn get_talent_tree(
        &self,
        talent_tree_id: u32,
        spec_id: u32,
    ) -> Result<TalentTreeResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/talent-tree/{talent_tree_id}/playable-specialization/{spec_id}"
                ),
                "static",
            )
            .await
    }

    pub async fn get_talent_tree_index(
        &self,
    ) -> Result<TalentTreeIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/talent-tree/index".to_string(), "static")
            .await
    }

    pub async fn get_talent_tree_nodes(
        &self,
        talent_tree_id: u32,
    ) -> Result<TalentTreeNodesResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/talent-tree/{talent_tree_id}/nodes"),
                "static",
            )
            .await
    }
}
