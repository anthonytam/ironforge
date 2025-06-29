use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterMediaSummaryResponse {
    pub character: Character,
    pub assets: Vec<Asset>,
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
pub struct Asset {
    pub key: String,
    pub value: String,
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
}

impl WorldOfWarcraftClient {
    pub async fn get_character_media_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterMediaSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/character-media"
                ),
                "profile",
            )
            .await
    }
}
