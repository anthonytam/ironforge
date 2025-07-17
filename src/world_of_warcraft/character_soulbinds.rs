use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::character_soulbinds::CharacterSoulbindsResponse};

impl WorldOfWarcraftClient {
    pub async fn get_character_soulbinds(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterSoulbindsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/soulbinds"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_soulbinds_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_soulbinds_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Soulbinds Functions ===");
        
        let result = client.get_character_soulbinds("zuljin", "panchäm").await;
        print_error(&result, "get_character_soulbinds");
    }
}