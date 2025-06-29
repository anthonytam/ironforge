use super::{WorldOfWarcraftClient, types::auction_house::Auctions};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_auctions(&self, id: u32) -> Result<Auctions, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/connected-realm/{id}/auctions"),
                "dynamic",
            )
            .await
    }

    pub async fn get_commodity_auctions(&self) -> Result<Auctions, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/auctions/commodities".to_string(), "dynamic")
            .await
    }
}
