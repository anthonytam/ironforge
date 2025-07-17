use super::{
    WorldOfWarcraftClient,
    types::guild_crest::GuildCrestComponentsIndexResponse,
};
use crate::{
    api_client::{ApiRequestHelper, BlizzardAPIClientError},
    world_of_warcraft::types::guild_crest::{
        GuildCrestBorderMedia, GuildCrestEmblemMedia, GuildCrestIndex,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_guild_crest_index(&self) -> Result<GuildCrestIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/guild-crest/index".to_string(), "static")
            .await
    }

    pub async fn get_guild_crest_border_media(
        &self,
        border_id: u32,
    ) -> Result<GuildCrestBorderMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/guild-crest/border/{border_id}"),
                "static",
            )
            .await
    }

    pub async fn get_guild_crest_emblem_media(
        &self,
        emblem_id: u32,
    ) -> Result<GuildCrestEmblemMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/guild-crest/emblem/{emblem_id}"),
                "static",
            )
            .await
    }

    pub async fn get_guild_crest_components_index(
        &self,
    ) -> Result<GuildCrestComponentsIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/guild-crest/index".to_string(),
                "static",
            )
            .await
    }
}

#[cfg(test)]
mod guild_crest_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_guild_crest_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Guild Crest Functions ===");
        
        let guild_crest_index = client.get_guild_crest_components_index().await;
        print_error(&guild_crest_index, "get_guild_crest_components_index");

        let guild_crest_index = guild_crest_index.unwrap();
        let guild_crest_border = guild_crest_index.borders.first().unwrap();
        let result = client.get_guild_crest_border_media(guild_crest_border.id).await;
        print_error(&result, "get_guild_crest_border_media");
        
        let guild_crest_emblem = guild_crest_index.emblems.first().unwrap();
        let result = client.get_guild_crest_emblem_media(guild_crest_emblem.id).await;
        print_error(&result, "get_guild_crest_emblem_media");

    }
}   