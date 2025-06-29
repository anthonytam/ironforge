use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_pvp::{CharacterPvpBracketStatisticsResponse, CharacterPvpSummaryResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_pvp_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterPvpSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/pvp-summary"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_pvp_bracket_statistics(
        &self,
        realm_slug: &str,
        character_name: &str,
        bracket: &str,
    ) -> Result<CharacterPvpBracketStatisticsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/pvp-bracket/{bracket}"
                ),
                "profile",
            )
            .await
    }
}
