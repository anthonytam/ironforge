use super::{
    WorldOfWarcraftClient,
    types::connected_realm::{ConnectedRealm, ConnectedRealmsIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_connected_realms_index(
        &self,
    ) -> Result<ConnectedRealmsIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/connected-realm/index".to_string(), "dynamic")
            .await
    }

    pub async fn get_connected_realm(
        &self,
        id: u32,
    ) -> Result<ConnectedRealm, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/connected-realm/{id}"), "dynamic")
            .await
    }
}

#[cfg(test)]
mod connected_realm_tests {
    use crate::world_of_warcraft::{test_utils::test_utils::{create_test_client, print_error}, types::connected_realm::ConnectedRealm};

    #[tokio::test]
    async fn test_connected_realm_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Connected Realm Functions ===");
        
        let connected_realm_index = client.get_connected_realms_index().await;
        print_error(&connected_realm_index, "get_connected_realms_index");

        let connected_realm_index = connected_realm_index.unwrap();
        let connected_realm = connected_realm_index.connected_realms.first().unwrap();
        let connected_realm_id = client.get_href_data::<ConnectedRealm>(connected_realm).await.unwrap().id;
        let result = client.get_connected_realm(connected_realm_id).await;

        print_error(&result, "get_connected_realm");
    }
}