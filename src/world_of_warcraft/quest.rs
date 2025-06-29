use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::quest::{
        QuestAreaIndexResponse, QuestAreaResponse, QuestCategoryIndexResponse,
        QuestCategoryResponse, QuestIndexResponse, QuestResponse, QuestTypeIndexResponse,
        QuestTypeResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_quest(&self, quest_id: u32) -> Result<QuestResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/quest/{quest_id}"), "static")
            .await
    }

    pub async fn get_quest_index(&self) -> Result<QuestIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/quest/index".to_string(), "static")
            .await
    }

    pub async fn get_quest_area(
        &self,
        quest_area_id: u32,
    ) -> Result<QuestAreaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/quest-area/{quest_area_id}"), "static")
            .await
    }

    pub async fn get_quest_area_index(
        &self,
    ) -> Result<QuestAreaIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/quest-area/index".to_string(), "static")
            .await
    }

    pub async fn get_quest_category(
        &self,
        quest_category_id: u32,
    ) -> Result<QuestCategoryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/quest-category/{quest_category_id}"),
                "static",
            )
            .await
    }

    pub async fn get_quest_category_index(
        &self,
    ) -> Result<QuestCategoryIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/quest-category/index".to_string(), "static")
            .await
    }

    pub async fn get_quest_type(
        &self,
        quest_type_id: u32,
    ) -> Result<QuestTypeResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/quest-type/{quest_type_id}"), "static")
            .await
    }

    pub async fn get_quest_type_index(
        &self,
    ) -> Result<QuestTypeIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/quest-type/index".to_string(), "static")
            .await
    }
}
