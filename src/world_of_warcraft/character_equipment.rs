use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::character_equipment::CharacterEquipmentSummaryResponse};

impl WorldOfWarcraftClient {
    pub async fn get_character_equipment_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterEquipmentSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/equipment"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_equipment_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_equipment_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Equipment Functions ===");
        
        let result = client.get_character_equipment_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_equipment_summary");
    }
}