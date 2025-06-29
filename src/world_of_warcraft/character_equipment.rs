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
