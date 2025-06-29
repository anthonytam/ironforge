use ironforge::api_client::{BlizzardAPIClientBuilder, Locale, Region};
use ironforge::world_of_warcraft::WorldOfWarcraftClient;

#[tokio::main]
async fn main() {
    // Create the API client
    let client = BlizzardAPIClientBuilder::new()
        .client_id("your_client_id")
        .client_secret("your_client_secret")
        .region(Region::US)
        .locale(Locale::en_US)
        .build()
        .unwrap();

    // Create WoW client
    let wow_client = WorldOfWarcraftClient::get(client);

    // Example: Get achievements index
    println!("Fetching achievements index...");
    let achievements = wow_client.get_achievements_index().await.unwrap();
    println!("Found {} achievements", achievements.achievements.len());

    // Example: Get a specific realm
    println!("Fetching realm information...");
    let realm = wow_client.get_realm("area-52".to_string()).await.unwrap();
    println!("Realm: {} (ID: {})", realm.name, realm.id);

    // Example: Get character profile
    println!("Fetching character profile...");
    let character = wow_client
        .get_character_profile_summary("area-52", "CharacterName")
        .await
        .unwrap();
    println!("Character: {} (Level {})", character.name, character.level);
}
