use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient, types::character_reputations::CharacterReputationsSummaryResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_character_reputations_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterReputationsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/reputations"
                ),
                "profile",
            )
            .await
    }
}
