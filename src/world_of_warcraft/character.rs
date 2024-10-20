use super::{types::character::CharacterEquipmentSummary, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_character_equipment_summary(&self, realm_slug: &str, character_name: &str) -> Result<CharacterEquipmentSummary> {
        //let response_result = self.client
                                //.send_request(format!("/profile/wow/character/{}/{}/equipment", realm_slug, character_name), "profile-us")
                                //.await;
                            //println!("{response_result:?}");
                            let response_result = self.client
                                .send_request(format!("/profile/wow/character/{}/{}/equipment", realm_slug, character_name), "profile-us")
                                .await;

                                let response_result = response_result
                                .json()
                                .await;

                            response_result.map_err(anyhow::Error::from)
        // match response_result {
        //     Ok(response) => Ok(response),
        //     Err(e) => Err(anyhow::Error::new(e)) //panic!("Failed to get a response. {:?}", e)
        // }
    }
}