use super::{types::auction_house::Auctions, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_auctions(&self, id: u32) -> Result<Auctions> {
        let response = self.client
                                .send_request(format!("/data/wow/connected-realm/{}/auctions", id), "dynamic")
                                .await?
                                .json::<Auctions>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_commodity_auctions(&self) -> Result<Auctions> {
        let response = self.client
                                .send_request(format!("/data/wow/auctions/commodities"), "dynamic")
                                .await?
                                .json::<Auctions>()
                                .await?;
        
                                Ok(response)
    }
}