use super::{
    WorldOfWarcraftClient,
    types::mythic_keystone_leaderboard::{
        MythicKeystoneLeaderboard, MythicKeystoneLeaderboardIndex,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_mythic_keystone_leaderboard_index(
        &self,
        connected_realm_id: u32,
    ) -> Result<MythicKeystoneLeaderboardIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/connected-realm/{connected_realm_id}/mythic-leaderboard/index"
                ),
                "dynamic",
            )
            .await
    }

    pub async fn get_mythic_keystone_leaderboard(
        &self,
        connected_realm_id: u32,
        dungeon_id: u32,
        period: u32,
    ) -> Result<MythicKeystoneLeaderboard, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/connected-realm/{connected_realm_id}/mythic-leaderboard/{dungeon_id}/period/{period}"
                ),
                "dynamic",
            )
            .await
    }
}
