use super::{types::guild_crest::{GuildCrestBorderMedia, GuildCrestEmblemMedia, GuildCrestIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_guild_crest_index(&self) -> Result<GuildCrestIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/index"), "static")
                                .await?
                                .json::<GuildCrestIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_guild_crest_border_media(&self, border_id: u32) -> Result<GuildCrestBorderMedia> {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/border/{}", border_id), "static")
                                .await?
                                .json::<GuildCrestBorderMedia>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_guild_crest_emblem_media(&self, emblem_id: u32) -> Result<GuildCrestEmblemMedia> {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/emblem/{}", emblem_id), "static")
                                .await?
                                .json::<GuildCrestEmblemMedia>()
                                .await;

                            response_result.map_err(anyhow::Error::from)
    }
}