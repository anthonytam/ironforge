use super::{types::toy::{Toy, ToyIndex}, world_of_warcraft_client::WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_toy_index(&self) -> ToyIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/toy/index"), "static")
                                .await
                                .json::<ToyIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_toy(&self, id: u32) -> Toy {
        let response_result = self.client
                                .send_request(format!("/data/wow/toy/{}", id), "static")
                                .await
                                .json::<Toy>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}