use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_profile::{CharacterProfileResponse, CharacterProfileStatusResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_profile_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfileResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/profile/wow/character/{realm_slug}/{character_name}"),
                "profile",
            )
            .await
    }

    pub async fn get_character_profile_status(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterProfileStatusResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/status"
                ),
                "profile",
            )
            .await
    }
}
