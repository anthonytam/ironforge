use super::{types::azerite_essence::{AzeriteEssenceDetail, AzeriteEssenceIndex, AzeriteEssenceMedia}, world_of_warcraft_client::WorldOfWarcraftClient};


impl WorldOfWarcraftClient {
    pub async fn get_azerite_essence_index(&self) -> AzeriteEssenceIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/azerite-essence/index"), "static")
                                .await
                                .json::<AzeriteEssenceIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_azerite_essence(&self, id: u32) -> AzeriteEssenceDetail {
        let response_result = self.client
                                .send_request(format!("/data/wow/azerite-essence/{}", id), "static")
                                .await
                                .json::<AzeriteEssenceDetail>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    // TODO: Implement Azerite Essence Search

    pub async fn get_azerite_essence_media(&self, id: u32) -> AzeriteEssenceMedia {
        let response_result = self.client
                                .send_request(format!("/data/wow/media/azerite-essence/{}", id), "static")
                                .await
                                .json::<AzeriteEssenceMedia>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}