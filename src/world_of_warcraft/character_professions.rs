use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient, types::character_professions::CharacterProfessionsSummaryResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_character_professions_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfessionsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/professions"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_professions_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_professions_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Professions Functions ===");
        
        let result = client.get_character_professions_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_professions_summary");
    }
}