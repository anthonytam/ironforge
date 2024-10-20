use super::{types::title::{Title, TitleIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_title_index(&self) -> Result<TitleIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/title/index"), "static")
                                .await?
                                .json::<TitleIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_title(&self, id: u32) -> Result<Title> {
        let response_result = self.client
                                .send_request(format!("/data/wow/title/{}", id), "static")
                                .await?
                                .json::<Title>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}