use super::{types::wow_token::WowTokenIndex, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_wow_token_index(&self) -> Result<WowTokenIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/token/index"), "dynamic")
                                .await?
                                .json::<WowTokenIndex>()
                                .await?;
        
                                Ok(response)
    }
}