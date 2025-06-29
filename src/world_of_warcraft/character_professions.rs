use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient, types::character_professions::CharacterProfessionsSummaryResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_character_professions_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfessionsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/professions"
                ),
                "profile",
            )
            .await
    }
}
