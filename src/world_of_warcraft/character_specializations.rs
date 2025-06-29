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
