use super::{types::wow_token::WowTokenIndex, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_wow_token_index(&self) -> WowTokenIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/token/index"), "dynamic")
                                .await
                                .json::<WowTokenIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
}