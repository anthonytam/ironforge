use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient, types::character_statistics::CharacterStatisticsSummaryResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_character_statistics_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterStatisticsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/statistics"
                ),
                "profile",
            )
            .await
    }
}
