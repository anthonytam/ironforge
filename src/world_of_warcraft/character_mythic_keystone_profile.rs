use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_mythic_keystone_profile::{
        CharacterMythicKeystoneProfileIndexResponse, CharacterMythicKeystoneSeasonDetailsResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_mythic_keystone_profile_index(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterMythicKeystoneProfileIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/mythic-keystone-profile"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_mythic_keystone_season_details(
        &self,
        realm_slug: &str,
        character_name: &str,
        season_id: u32,
    ) -> Result<CharacterMythicKeystoneSeasonDetailsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/mythic-keystone-profile/season/{season_id}"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_mythic_keystone_profile_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_mythic_keystone_profile_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Mythic Keystone Profile Functions ===");
        
        let keystone_index = client.get_character_mythic_keystone_profile_index("zuljin", "panchäm").await;
        print_error(&keystone_index, "get_character_mythic_keystone_profile_index");

        let season_id = keystone_index.unwrap().seasons.first().unwrap().id;
        let season_details = client.get_character_mythic_keystone_season_details("zuljin", "panchäm", season_id).await;
        print_error(&season_details, "get_character_mythic_keystone_season_details");
    }
}