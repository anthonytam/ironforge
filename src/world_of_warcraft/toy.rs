use super::{types::toy::{Toy, ToyIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_toy_index(&self) -> Result<ToyIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/toy/index"), "static")
                                .await?
                                .json::<ToyIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_toy(&self, id: u32) -> Result<Toy> {
        let response_result = self.client
                                .send_request(format!("/data/wow/toy/{}", id), "static")
                                .await?
                                .json::<Toy>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}