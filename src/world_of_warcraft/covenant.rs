use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::covenant::*};

impl WorldOfWarcraftClient {
    pub async fn get_conduit(
        &self,
        conduit_id: u32,
    ) -> Result<ConduitResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/covenant/conduit/{conduit_id}"),
                "static",
            )
            .await
    }

    pub async fn get_conduit_index(&self) -> Result<ConduitIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/conduit/index".to_string(), "static")
            .await
    }

    pub async fn get_covenant(
        &self,
        covenant_id: u32,
    ) -> Result<CovenantResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/covenant/{covenant_id}"), "static")
            .await
    }

    pub async fn get_covenant_index(&self) -> Result<CovenantIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/index".to_string(), "static")
            .await
    }

    pub async fn get_covenant_media(
        &self,
        covenant_id: u32,
    ) -> Result<CovenantMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/covenant/{covenant_id}"),
                "static",
            )
            .await
    }

    pub async fn get_soulbind(
        &self,
        soulbind_id: u32,
    ) -> Result<SoulbindResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/covenant/soulbind/{soulbind_id}"),
                "static",
            )
            .await
    }

    pub async fn get_soulbind_index(&self) -> Result<SoulbindIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/covenant/soulbind/index".to_string(), "static")
            .await
    }
}

#[cfg(test)]
mod covenant_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_covenant_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Covenant Functions ===");
        
        let covenant_index = client.get_covenant_index().await;
        print_error(&covenant_index, "get_covenants_index");
        
        let covenant_index = covenant_index.unwrap();
        let covenant = covenant_index.covenants.first().unwrap();
        let result = client.get_covenant(covenant.id).await;
        print_error(&result, "get_covenant");
        
        let result = client.get_covenant_media(covenant.id).await;
        print_error(&result, "get_covenant_media");
        
        let soulbind_index = client.get_soulbind_index().await;
        print_error(&soulbind_index, "get_soulbinds_index");
        
        let soulbind_index = soulbind_index.unwrap();
        let soulbind = soulbind_index.soulbinds.first().unwrap();
        let result = client.get_soulbind(soulbind.id).await;
        print_error(&result, "get_soulbind");
        
        let conduit_index = client.get_conduit_index().await;
        print_error(&conduit_index, "get_conduits_index");
        
        let conduit_index = conduit_index.unwrap();
        let conduit = conduit_index.conduits.first().unwrap();
        let result = client.get_conduit(conduit.id).await;
        print_error(&result, "get_conduit");
    }
}