use super::{types::heirloom::{Heirloom, HeirloomIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_heirloom_index(&self) -> Result<HeirloomIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/heirloom/index"), "static")
                                .await?
                                .json::<HeirloomIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_heirloom(&self, id: u32) -> Result<Heirloom> {
        let response = self.client
                                .send_request(format!("/data/wow/heirloom/{}", id), "static")
                                .await?
                                .json::<Heirloom>()
                                .await?;
        
                                Ok(response)
    }
}