use super::{types::title::{Title, TitleIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_title_index(&self) -> TitleIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/title/index"), "static")
                                .await
                                .json::<TitleIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_title(&self, id: u32) -> Title {
        let response_result = self.client
                                .send_request(format!("/data/wow/title/{}", id), "static")
                                .await
                                .json::<Title>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
}