use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::guild::*};

impl WorldOfWarcraftClient {
    pub async fn get_guild(
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

    pub async fn get_guild_activity(
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

    pub async fn get_guild_achievements(
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

    pub async fn get_guild_roster(
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

#[cfg(test)]
mod guild_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_guild_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Guild Functions ===");
        
        let result = client.get_guild("zuljin", "all-bread-no-meat").await;
        print_error(&result, "get_guild");
        
        let result = client.get_guild_activity("zuljin", "all-bread-no-meat").await;
        print_error(&result, "get_guild_activity");
        
        let result = client.get_guild_achievements("zuljin", "all-bread-no-meat").await;
        print_error(&result, "get_guild_achievements");
        
        let result = client.get_guild_roster("zuljin", "all-bread-no-meat").await;
        print_error(&result, "get_guild_roster");
    }
}   