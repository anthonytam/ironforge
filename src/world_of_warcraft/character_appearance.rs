use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterAppearanceSummaryResponse {
    pub character: Character,
    pub items: Vec<Item>,
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
pub struct Item {
    pub item: ItemSummary,
    pub slot: Slot,
    pub enchant: Option<u32>,
    pub item_appearance: Option<ItemAppearance>,
    pub transmog: Option<Transmog>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Slot {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemAppearance {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transmog {
    pub item: ItemSummary,
    pub display_string: String,
}

impl WorldOfWarcraftClient {
    pub async fn get_character_appearance_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterAppearanceSummaryResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/appearance"
                ),
                "profile",
            )
            .await
    }
}
