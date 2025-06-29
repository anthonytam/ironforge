use super::{
    WorldOfWarcraftClient,
    types::guild_crest::{GuildCrestBorderEmblemResponse, GuildCrestComponentsIndexResponse},
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
                format!("/data/wow/guild-crest/border/{border_id}"),
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
                format!("/data/wow/guild-crest/emblem/{emblem_id}"),
                "static",
            )
            .await
    }

    pub async fn get_guild_crest_border(
        &self,
        border_id: u32,
    ) -> Result<GuildCrestBorderEmblemResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild-crest/border/{border_id}"),
                "static",
            )
            .await
    }

    pub async fn get_guild_crest_emblem(
        &self,
        emblem_id: u32,
    ) -> Result<GuildCrestBorderEmblemResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/guild-crest/emblem/{emblem_id}"),
                "static",
            )
            .await
    }

    pub async fn get_guild_crest_components_index(
        &self,
    ) -> Result<GuildCrestComponentsIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/guild-crest/components/index".to_string(),
                "static",
            )
            .await
    }
}
