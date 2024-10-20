use super::{types::toy::{Toy, ToyIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_toy_index(&self) -> Result<ToyIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/toy/index"), "static")
                                .await?
                                .json::<ToyIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_toy(&self, id: u32) -> Result<Toy> {
        let response = self.client
                                .send_request(format!("/data/wow/toy/{}", id), "static")
                                .await?
                                .json::<Toy>()
                                .await?;
        
                                Ok(response)
    }
}