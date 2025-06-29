use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::achievements::{
        Achievement, AchievementCategoryIndex, AchievementMedia, AchievementSummaryCategory,
        AchievementsIndex,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_achievement_categories_index(
        &self,
    ) -> Result<AchievementCategoryIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/achievement-category/index".to_string(), "static")
            .await
    }

    pub async fn get_achievement_category(
        &self,
        id: u32,
    ) -> Result<AchievementSummaryCategory, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/achievement-category/{id}"), "static")
            .await
    }

    pub async fn get_achievements_index(
        &self,
    ) -> Result<AchievementsIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/achievement/index".to_string(), "static")
            .await
    }

    pub async fn get_achievement(&self, id: u32) -> Result<Achievement, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/achievement/{id}"), "static")
            .await
    }

    pub async fn get_achievement_media(
        &self,
        id: u32,
    ) -> Result<AchievementMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/achievement/{id}"), "static")
            .await
    }
}
