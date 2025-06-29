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
