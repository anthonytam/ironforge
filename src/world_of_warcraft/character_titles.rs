use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::character_titles::CharacterTitlesSummaryResponse};

impl WorldOfWarcraftClient {
    pub async fn get_character_titles_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterTitlesSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/titles"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_titles_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_titles_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Titles Functions ===");
        
        let result = client.get_character_titles_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_titles_summary");
    }
}   