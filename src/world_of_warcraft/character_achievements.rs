use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_achievements::{
        CharacterAchievementStatisticsResponse, CharacterAchievementsSummaryResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_achievements_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterAchievementsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/achievements"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_achievement_statistics(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterAchievementStatisticsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/achievements/statistics"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_achievements_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_achievements_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Achievements Functions ===");
        
        let result = client.get_character_achievements_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_achievements_summary");
        
        let result = client.get_character_achievement_statistics("zuljin", "panchäm").await;
        print_error(&result, "get_character_achievement_statistics");
    }
}