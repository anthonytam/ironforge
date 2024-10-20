use super::{types::heirloom::{Heirloom, HeirloomIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_heirloom_index(&self) -> Result<HeirloomIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/heirloom/index"), "static")
                                .await?
                                .json::<HeirloomIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_heirloom(&self, id: u32) -> Result<Heirloom> {
        let response_result = self.client
                                .send_request(format!("/data/wow/heirloom/{}", id), "static")
                                .await?
                                .json::<Heirloom>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}