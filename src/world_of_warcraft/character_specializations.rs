use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_specializations::CharacterSpecializationsSummaryResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_character_specializations_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterSpecializationsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/specializations"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_specializations_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_specializations_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Specializations Functions ===");
        
        let result = client.get_character_specializations_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_specializations_summary");
    }
}