use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_pvp::{CharacterPvpBracketStatisticsResponse, CharacterPvpSummaryResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_pvp_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterPvpSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/pvp-summary"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_pvp_bracket_statistics(
        &self,
        realm_slug: &str,
        character_name: &str,
        bracket: &str,
    ) -> Result<CharacterPvpBracketStatisticsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/pvp-bracket/{bracket}"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_pvp_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_pvp_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character PVP Functions ===");
        
        let result = client.get_character_pvp_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_pvp_summary");

        let result = client.get_character_pvp_bracket_statistics("zuljin", "panchäm", "2v2").await;
        print_error(&result, "get_character_pvp_bracket_statistics");
    }
}