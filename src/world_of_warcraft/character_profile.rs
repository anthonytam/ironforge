use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_profile::{CharacterProfileResponse, CharacterProfileStatusResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_profile_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfileResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/profile/wow/character/{realm_slug}/{character_name}"),
                "profile",
            )
            .await
    }

    pub async fn get_character_profile_status(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfileStatusResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/status"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_profile_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_profile_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Profile Functions ===");
        
        let result = client.get_character_profile_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_profile_summary");

        let result = client.get_character_profile_status("zuljin", "panchäm").await;
        print_error(&result, "get_character_profile_status");
    }
}