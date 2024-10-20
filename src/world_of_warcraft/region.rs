use super::{types::region::{Region, RegionIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_region_index(&self) -> Result<RegionIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/region/index"), "dynamic")
                                .await?
                                .json::<RegionIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_region(&self, id: u32) -> Result<Region> {
        let response = self.client
                                .send_request(format!("/data/wow/region/{}", id), "dynamic")
                                .await?
                                .json::<Region>()
                                .await?;
        
                                Ok(response)
    }
}