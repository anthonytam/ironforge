use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::tech_talent::{
        TechTalentIndexResponse, TechTalentMediaResponse, TechTalentResponse,
        TechTalentTreeIndexResponse, TechTalentTreeResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_tech_talent(
        &self,
        tech_talent_id: u32,
    ) -> Result<TechTalentResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/tech-talent/{tech_talent_id}"),
                "static",
            )
            .await
    }

    pub async fn get_tech_talent_index(
        &self,
    ) -> Result<TechTalentIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/tech-talent/index".to_string(), "static")
            .await
    }

    pub async fn get_tech_talent_media(
        &self,
        tech_talent_id: u32,
    ) -> Result<TechTalentMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/tech-talent/{tech_talent_id}"),
                "static",
            )
            .await
    }

    pub async fn get_tech_talent_tree(
        &self,
        tech_talent_tree_id: u32,
    ) -> Result<TechTalentTreeResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/tech-talent-tree/{tech_talent_tree_id}"),
                "static",
            )
            .await
    }

    pub async fn get_tech_talent_tree_index(
        &self,
    ) -> Result<TechTalentTreeIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/tech-talent-tree/index".to_string(), "static")
            .await
    }
}
