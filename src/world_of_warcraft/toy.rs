use super::{types::toy::{Toy, ToyIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_toy_index(&self) -> ToyIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/toy/index"), "static")
                                .await
                                .json::<ToyIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
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
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
}