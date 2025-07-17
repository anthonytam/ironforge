use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_encounters::{
        CharacterDungeonsResponse, CharacterEncountersSummaryResponse, CharacterRaidsResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_encounters_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterEncountersSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_dungeons(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterDungeonsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters/dungeons"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_raids(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterRaidsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters/raids"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_encounters_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_collections_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Encounters Functions ===");
        
        let result = client.get_character_encounters_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_encounters_summary");

        let result = client.get_character_dungeons("zuljin", "panchäm").await;
        print_error(&result, "get_character_dungeons");

        let result = client.get_character_raids("zuljin", "panchäm").await;
        print_error(&result, "get_character_raids");
    }
}