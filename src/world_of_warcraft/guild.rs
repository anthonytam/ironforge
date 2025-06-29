use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::guild::*};

impl WorldOfWarcraftClient {
    pub async fn guild(
        &self,
        realm_slug: &str,
        name_slug: &str,
    ) -> Result<GuildResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild/{realm_slug}/{name_slug}"),
                "profile",
            )
            .await
    }

    pub async fn guild_activity(
        &self,
        realm_slug: &str,
        name_slug: &str,
    ) -> Result<GuildActivityResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild/{realm_slug}/{name_slug}/activity"),
                "profile",
            )
            .await
    }

    pub async fn guild_achievements(
        &self,
        realm_slug: &str,
        name_slug: &str,
    ) -> Result<GuildAchievementsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild/{realm_slug}/{name_slug}/achievements"),
                "profile",
            )
            .await
    }

    pub async fn guild_roster(
        &self,
        realm_slug: &str,
        name_slug: &str,
    ) -> Result<GuildRosterResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild/{realm_slug}/{name_slug}/roster"),
                "profile",
            )
            .await
    }
}
