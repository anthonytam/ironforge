use super::{types::creature::{CreatureFamily, CreatureFamilyIndex, CreatureType}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_creature_family_index(&self) -> CreatureFamilyIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-family/index"), "static")
                                .await
                                .json::<CreatureFamilyIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_creature_family(&self, id: u32) -> CreatureFamily {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-family/{}", id), "static")
                                .await
                                .json::<CreatureFamily>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_creature_types_index(&self) -> CreatureFamilyIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-type/index"), "static")
                                .await
                                .json::<CreatureFamilyIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_creature_type(&self, id: u32) -> CreatureType {
        let response_result = self.client
                                .send_request(format!("/data/wow/creature-type/{}", id), "static")
                                .await
                                .json::<CreatureType>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}