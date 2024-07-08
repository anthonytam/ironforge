use super::{types::region::{Region, RegionIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_region_index(&self) -> RegionIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/region/index"), "dynamic")
                                .await
                                .json::<RegionIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_region(&self, id: u32) -> Region {
        let response_result = self.client
                                .send_request(format!("/data/wow/region/{}", id), "dynamic")
                                .await
                                .json::<Region>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}