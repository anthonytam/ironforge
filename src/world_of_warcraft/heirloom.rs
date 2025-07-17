use super::{
    WorldOfWarcraftClient,
    types::heirloom::{Heirloom, HeirloomIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_heirloom_index(&self) -> Result<HeirloomIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/heirloom/index".to_string(), "static")
            .await
    }

    pub async fn get_heirloom(&self, id: u32) -> Result<Heirloom, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/heirloom/{id}"), "static")
            .await
    }
}

#[cfg(test)]
mod heirloom_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_heirloom_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Heirloom Functions ===");
        
        let heirloom_index = client.get_heirloom_index().await;
        print_error(&heirloom_index, "get_heirloom_index");
        
        let heirloom_index = heirloom_index.unwrap();
        let heirloom = heirloom_index.heirlooms.first().unwrap();
        let result = client.get_heirloom(heirloom.id).await;
        print_error(&result, "get_heirloom");
    }
}   