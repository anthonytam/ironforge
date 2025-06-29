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
