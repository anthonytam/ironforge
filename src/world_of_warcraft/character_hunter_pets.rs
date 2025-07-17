use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

// TODO: Move these to types
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterHunterPetsSummaryResponse {
    pub character: Character,
    pub hunter_pets: Vec<HunterPet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub key: Key,
    pub name: String,
    pub id: u32,
    pub realm: Realm,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Realm {
    pub key: Key,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HunterPet {
    pub name: String,
    pub level: u32,
    pub creature: Creature,
    pub slot: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Creature {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

impl WorldOfWarcraftClient {
    pub async fn get_character_hunter_pets_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterHunterPetsSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/hunter-pets"
                ),
                "profile",
            )
            .await
    }
}

#[cfg(test)]
mod character_hunter_pets_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_hunter_pets_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Hunter Pets Functions ===");
        
        let result = client.get_character_hunter_pets_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_hunter_pets_summary");
    }
}