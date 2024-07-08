use super::{types::guild_crest::{GuildCrestBorderMedia, GuildCrestEmblemMedia, GuildCrestIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_guild_crest_index(&self) -> GuildCrestIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/index"), "static")
                                .await
                                .json::<GuildCrestIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_guild_crest_border_media(&self, border_id: u32) -> GuildCrestBorderMedia {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/border/{}", border_id), "static")
                                .await
                                .json::<GuildCrestBorderMedia>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_guild_crest_emblem_media(&self, emblem_id: u32) -> GuildCrestEmblemMedia {
        let response_result = self.client
                                .send_request(format!("/data/wow/guild-crest/emblem/{}", emblem_id), "static")
                                .await
                                .json::<GuildCrestEmblemMedia>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}