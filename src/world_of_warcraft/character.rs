use super::{types::character::CharacterEquipmentSummary, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_character_equipment_summary(&self, realm_slug: &str, character_name: &str) -> Result<CharacterEquipmentSummary> {
                            let response_result = self.client
                                .send_request(format!("/profile/wow/character/{}/{}/equipment", realm_slug, character_name), "profile")
                                .await
                                .json::<CharacterEquipmentSummary>()
                                .await?;
                            
                            Ok(response_result)
    }
}