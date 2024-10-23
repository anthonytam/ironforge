use dotenv::dotenv;
use std::env;
use tokio;

use crate::{api_client::{BlizzardAPIClient, Locale, Region}, world_of_warcraft::WorldOfWarcraftClient};

// Basic test ensures a Blizzard API client can be created, a WoWClient can be created, and an API call can be made.
#[tokio::test]
async fn get_character_equipment_summary_basic_test() {
    dotenv().ok();

    let blizzard_client = BlizzardAPIClient::new(env::var("BLIZZARD_CLIENT_ID").unwrap(),
                                                                env::var("BLIZZARD_CLIENT_SECRET").unwrap(),
                                                                Region::EU,
                                                                Locale::en_US);

    let wow_client = WorldOfWarcraftClient::get(blizzard_client);
    let armory_character_equipment_summary_result = wow_client.get_character_equipment_summary("ragnaros", "armory").await;

    if let Ok(armory_character_equipment_summary) = armory_character_equipment_summary_result {
        assert!(armory_character_equipment_summary.character.name == "Armory");
    } else {
        panic!("{:?}", armory_character_equipment_summary_result);
    }
}
