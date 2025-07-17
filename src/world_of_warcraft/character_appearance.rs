use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};
use serde::{Deserialize, Serialize};

use super::WorldOfWarcraftClient;
// TODO: Move these to types
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


#[cfg(test)]
mod character_appearance_tests {
    use crate::world_of_warcraft::test_utils::test_utils::{create_test_client, print_error};

    #[tokio::test]
    async fn test_character_appearance_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Character Appearance Functions ===");
        
        let result = client.get_character_appearance_summary("zuljin", "panchäm").await;
        print_error(&result, "get_character_appearance_summary");
    }
}