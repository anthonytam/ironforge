use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_collections::{
        CharacterCollectionsIndexResponse, CharacterHeirloomsCollectionSummaryResponse,
        CharacterMountsCollectionSummaryResponse, CharacterPetsCollectionSummaryResponse,
        CharacterToysCollectionSummaryResponse, CharacterTransmogCollectionSummaryResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_character_collections_index(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterCollectionsIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_mounts_collection_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterMountsCollectionSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections/mounts"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_pets_collection_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterPetsCollectionSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections/pets"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_toys_collection_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterToysCollectionSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections/toys"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_heirlooms_collection_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterHeirloomsCollectionSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections/heirlooms"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_transmog_collection_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterTransmogCollectionSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/collections/transmog"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_collections_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_collections_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Collections Functions ===");
        
        let result = client.get_character_collections_index("zuljin", "panchäm").await;
        print_error(&result, "get_character_collections_index");
        
        let result = client.get_character_mounts_collection_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_mounts_collection_summary");
        
        let result = client.get_character_pets_collection_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_pets_collection_summary");

        let result = client.get_character_toys_collection_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_toys_collection_summary");

        let result = client.get_character_heirlooms_collection_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_heirlooms_collection_summary");

        let result = client.get_character_transmog_collection_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_transmog_collection_summary");
    }
}