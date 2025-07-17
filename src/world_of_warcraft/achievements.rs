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

#[cfg(test)]
mod achievement_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_achievement_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Achievement Functions ===");
        
        let index_result = client.get_achievements_index().await;
        print_error(&index_result, "get_achievements_index");
        
        let achievement_id = index_result.unwrap().achievements.first().unwrap().id;
        let achievement_result = client.get_achievement(achievement_id).await;
        print_error(&achievement_result, "get_achievement");
        
        let media_id = achievement_result.unwrap().media.id;
        let media_result = client.get_achievement_media(media_id).await;
        print_error(&media_result, "get_achievement_media");
        
        let index_result = client.get_achievement_categories_index().await;
        print_error(&index_result, "get_achievement_categories_index");
        
        let category_id = index_result.unwrap().categories.first().unwrap().id;
        let category_result = client.get_achievement_category(category_id).await;
        print_error(&category_result, "get_achievement_category");
    }
}