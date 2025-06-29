use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_achievements::{
        CharacterAchievementStatisticsResponse, CharacterAchievementsSummaryResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_achievements_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterAchievementsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/achievements"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_achievement_statistics(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterAchievementStatisticsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/achievements/statistics"
                ),
                "profile",
            )
            .await
    }
}
