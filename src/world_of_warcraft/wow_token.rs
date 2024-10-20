use super::{types::wow_token::WowTokenIndex, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_wow_token_index(&self) -> Result<WowTokenIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/token/index"), "dynamic")
                                .await?
                                .json::<WowTokenIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}