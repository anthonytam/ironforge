use super::{types::creature::{CreatureFamily, CreatureFamilyIndex, CreatureType}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_creature_family_index(&self) -> Result<CreatureFamilyIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/creature-family/index"), "static")
                                .await?
                                .json::<CreatureFamilyIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_creature_family(&self, id: u32) -> Result<CreatureFamily> {
        let response = self.client
                                .send_request(format!("/data/wow/creature-family/{}", id), "static")
                                .await?
                                .json::<CreatureFamily>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_creature_types_index(&self) -> Result<CreatureFamilyIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/creature-type/index"), "static")
                                .await?
                                .json::<CreatureFamilyIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_creature_type(&self, id: u32) -> Result<CreatureType> {
        let response = self.client
                                .send_request(format!("/data/wow/creature-type/{}", id), "static")
                                .await?
                                .json::<CreatureType>()
                                .await?;
        
                                Ok(response)
    }
}