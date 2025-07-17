use super::{WorldOfWarcraftClient, types::auction_house::Auctions};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_auctions(&self, realm_id: u32) -> Result<Auctions, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/connected-realm/{realm_id}/auctions"),
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

#[cfg(test)]
mod auction_house_tests {
    use crate::world_of_warcraft::{test_utils::test_utils::{create_test_client, print_error}, types::connected_realm::ConnectedRealm};

    #[tokio::test]
    async fn test_auction_house_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Auction House Functions ===");
        
        let connected_realm_index = client.get_connected_realms_index().await.unwrap();
        let connected_realm = connected_realm_index.connected_realms.first().unwrap();
        let connected_realm_id = client.get_href_data::<ConnectedRealm>(connected_realm).await.unwrap().id;
        let result = client.get_auctions(connected_realm_id).await;
        print_error(&result, "get_auctions");

        let commodities_result = client.get_commodity_auctions().await;
        print_error(&commodities_result, "get_commodity_auctions");
    }
}