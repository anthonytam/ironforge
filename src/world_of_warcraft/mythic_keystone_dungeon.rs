use super::{
    WorldOfWarcraftClient,
    types::mythic_keystone_dungeon::{MythicKeystoneDungeon, MythicKeystoneDungeonIndex},
    types::mythic_keystone_dungeon::{
        MythicKeystoneIndex, MythicKeystonePeriod, MythicKeystonePeriodIndex, MythicKeystoneSeason,
        MythicKeystoneSeasonIndex,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_mythic_keystone_dungeon_index(
        &self,
    ) -> Result<MythicKeystoneDungeonIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/mythic-keystone/dungeon/index".to_string(),
                "static",
            )
            .await
    }

    pub async fn get_mythic_keystone_dungeon(
        &self,
        dungeon_id: u32,
    ) -> Result<MythicKeystoneDungeon, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/mythic-keystone/dungeon/{dungeon_id}"),
                "static",
            )
            .await
    }

    pub async fn get_mythic_keystone_index(
        &self,
    ) -> Result<MythicKeystoneIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/mythic-keystone/index".to_string(), "static")
            .await
    }

    pub async fn get_mythic_keystone_period_index(
        &self,
    ) -> Result<MythicKeystonePeriodIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/mythic-keystone/period/index".to_string(),
                "static",
            )
            .await
    }

    pub async fn get_mythic_keystone_period(
        &self,
        period_id: u32,
    ) -> Result<MythicKeystonePeriod, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/mythic-keystone/period/{period_id}"),
                "static",
            )
            .await
    }

    pub async fn get_mythic_keystone_season_index(
        &self,
    ) -> Result<MythicKeystoneSeasonIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/mythic-keystone/season/index".to_string(),
                "static",
            )
            .await
    }

    pub async fn get_mythic_keystone_season(
        &self,
        season_id: u32,
    ) -> Result<MythicKeystoneSeason, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/mythic-keystone/season/{season_id}"),
                "static",
            )
            .await
    }
}
