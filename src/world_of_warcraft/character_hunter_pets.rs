use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

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
