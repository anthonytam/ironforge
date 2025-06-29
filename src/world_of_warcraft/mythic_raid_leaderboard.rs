use super::{WorldOfWarcraftClient, types::mythic_raid_leaderboard::MythicRaidLeaderboard};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_mythic_raid_leaderboard(
        &self,
        raid_name: &str,
        faction: &str,
    ) -> Result<MythicRaidLeaderboard, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/leaderboard/hall-of-fame/{raid_name}/{faction}"),
                "dynamic",
            )
            .await
    }
}
