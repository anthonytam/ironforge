use super::{types::heirloom::{Heirloom, HeirloomIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_heirloom_index(&self) -> HeirloomIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/heirloom/index"), "static")
                                .await
                                .json::<HeirloomIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_heirloom(&self, id: u32) -> Heirloom {
        let response_result = self.client
                                .send_request(format!("/data/wow/heirloom/{}", id), "static")
                                .await
                                .json::<Heirloom>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
}