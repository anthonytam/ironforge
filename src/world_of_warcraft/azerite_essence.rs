use super::{types::azerite_essence::{AzeriteEssenceDetail, AzeriteEssenceIndex, AzeriteEssenceMedia}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_azerite_essence_index(&self) -> Result<AzeriteEssenceIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/azerite-essence/index"), "static")
                                .await?
                                .json::<AzeriteEssenceIndex>()
                                .await?;

                            Ok(response)
    }

    pub async fn get_azerite_essence(&self, id: u32) -> Result<AzeriteEssenceDetail> {
        let response = self.client
                                .send_request(format!("/data/wow/azerite-essence/{}", id), "static")
                                .await?
                                .json::<AzeriteEssenceDetail>()
                                .await?;
        
                                Ok(response)
    }

    // TODO: Implement Azerite Essence Search

    pub async fn get_azerite_essence_media(&self, id: u32) -> Result<AzeriteEssenceMedia> {
        let response = self.client
                                .send_request(format!("/data/wow/media/azerite-essence/{}", id), "static")
                                .await?
                                .json::<AzeriteEssenceMedia>()
                                .await?;
                            
                                Ok(response)
    }
}