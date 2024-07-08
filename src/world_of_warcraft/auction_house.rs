use super::{types::auction_house::Auctions, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_auctions(&self, id: u32) -> Auctions {
        let response_result = self.client
                                .send_request(format!("/data/wow/connected-realm/{}/auctions", id), "dynamic")
                                .await
                                .json::<Auctions>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_commodity_auctions(&self) -> Auctions {
        let response_result = self.client
                                .send_request(format!("/data/wow/auctions/commodities"), "dynamic")
                                .await
                                .json::<Auctions>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}