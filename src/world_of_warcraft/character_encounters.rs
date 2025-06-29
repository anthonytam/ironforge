use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_encounters::{
        CharacterDungeonsResponse, CharacterEncountersSummaryResponse, CharacterRaidsResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_encounters_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterEncountersSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_dungeons(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterDungeonsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters/dungeons"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_raids(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterRaidsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/encounters/raids"
                ),
                "profile",
            )
            .await
    }
}
