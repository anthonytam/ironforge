use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::pet::{
        PetAbilityIndexResponse, PetAbilityMediaResponse, PetAbilityResponse, PetIndexResponse,
        PetMediaResponse, PetResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_pet(&self, pet_id: u32) -> Result<PetResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/pet/{pet_id}"), "static")
            .await
    }

    pub async fn get_pet_index(&self) -> Result<PetIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/pet/index".to_string(), "static")
            .await
    }

    pub async fn get_pet_media(
        &self,
        pet_id: u32,
    ) -> Result<PetMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/pet/{pet_id}"), "static")
            .await
    }

    pub async fn get_pet_ability(
        &self,
        pet_ability_id: u32,
    ) -> Result<PetAbilityResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/pet-ability/{pet_ability_id}"),
                "static",
            )
            .await
    }

    pub async fn get_pet_ability_index(
        &self,
    ) -> Result<PetAbilityIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/pet-ability/index".to_string(), "static")
            .await
    }

    pub async fn get_pet_ability_media(
        &self,
        pet_ability_id: u32,
    ) -> Result<PetAbilityMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/pet-ability/{pet_ability_id}"),
                "static",
            )
            .await
    }
}
