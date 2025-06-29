use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::pvp_season::{
        PvpLeaderboardIndexResponse, PvpLeaderboardResponse, PvpRewardsIndexResponse,
        PvpSeasonIndexResponse, PvpSeasonResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_pvp_season(
        &self,
        pvp_season_id: u32,
    ) -> Result<PvpSeasonResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/pvp-season/{pvp_season_id}"), "dynamic")
            .await
    }

    pub async fn get_pvp_season_index(
        &self,
    ) -> Result<PvpSeasonIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/pvp-season/index".to_string(), "dynamic")
            .await
    }

    pub async fn get_pvp_leaderboard(
        &self,
        pvp_season_id: u32,
        bracket: &str,
    ) -> Result<PvpLeaderboardResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/pvp-season/{pvp_season_id}/pvp-leaderboard/{bracket}"
                ),
                "dynamic",
            )
            .await
    }

    pub async fn get_pvp_leaderboard_index(
        &self,
        pvp_season_id: u32,
    ) -> Result<PvpLeaderboardIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/pvp-season/{pvp_season_id}/pvp-leaderboard/index"
                ),
                "dynamic",
            )
            .await
    }

    pub async fn get_pvp_rewards_index(
        &self,
        pvp_season_id: u32,
    ) -> Result<PvpRewardsIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/pvp-season/{pvp_season_id}/pvp-reward/index"),
                "dynamic",
            )
            .await
    }
}
