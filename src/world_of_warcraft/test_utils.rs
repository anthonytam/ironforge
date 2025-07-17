#[cfg(test)]
pub mod test_utils {
    use crate::{api_client::{BlizzardAPIClient, Locale, Region}, world_of_warcraft::WorldOfWarcraftClient};
    use dotenv::dotenv;
    use std::env;

    pub async fn create_test_client() -> WorldOfWarcraftClient {
        dotenv().ok();
        
        let blizzard_client = BlizzardAPIClient::builder()
            .client_id(env::var("BLIZZARD_CLIENT_ID").expect("BLIZZARD_CLIENT_ID is not set."))
            .client_secret(env::var("BLIZZARD_CLIENT_SECRET").expect("BLIZZARD_CLIENT_SECRET is not set."))
            .region(Region::US)
            .locale(Locale::en_US)
            .build()
            .expect("Failed to create BlizzardAPIClient");

        WorldOfWarcraftClient::get(blizzard_client)
    }

    pub fn print_error<T>(result: &Result<T, crate::api_client::BlizzardAPIClientError>, function_name: &str) {
        match result {
            Ok(_) => println!("✅ {}: Success", function_name),
            Err(e) => {
                println!("❌ {}: Failed - {:?}", function_name, e);
                if let crate::api_client::BlizzardAPIClientError::DeserializationError(msg) = &e {
                    println!("   Deserialization Error: {}", msg);
                }
            }
        }
    }
} 