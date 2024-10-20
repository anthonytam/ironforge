use super::{types::creature::{CreatureFamily, CreatureFamilyIndex, CreatureType}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_creature_family_index(&self) -> Result<CreatureFamilyIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-family/index"), "static")
                                .await?
                                .json::<CreatureFamilyIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_creature_family(&self, id: u32) -> Result<CreatureFamily> {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-family/{}", id), "static")
                                .await?
                                .json::<CreatureFamily>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_creature_types_index(&self) -> Result<CreatureFamilyIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-type/index"), "static")
                                .await?
                                .json::<CreatureFamilyIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_creature_type(&self, id: u32) -> Result<CreatureType> {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-type/{}", id), "static")
                                .await?
                                .json::<CreatureType>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}