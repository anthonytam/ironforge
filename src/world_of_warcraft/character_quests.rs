use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_quests::{CharacterCompletedQuestsResponse, CharacterQuestsResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_quests(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterQuestsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/quests"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_completed_quests(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterCompletedQuestsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/quests/completed"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_quests_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_quests_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Quests Functions ===");
        
        let result = client.get_character_quests("zuljin", "panchäm").await;
        print_error(&result, "get_character_quests");

        let result = client.get_character_completed_quests("zuljin", "panchäm").await;
        print_error(&result, "get_character_completed_quests");
    }
}