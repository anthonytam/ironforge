use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::character_titles::CharacterTitlesSummaryResponse};

impl WorldOfWarcraftClient {
    pub async fn get_character_titles_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterTitlesSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/titles"
                ),
                "profile",
            )
            .await
    }
}
