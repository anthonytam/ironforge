use crate::api_client::BlizzardAPIClient;


pub struct WorldOfWarcraftClient {
    pub client: BlizzardAPIClient
}

impl WorldOfWarcraftClient {
    pub fn get (client: BlizzardAPIClient) -> WorldOfWarcraftClient {
        WorldOfWarcraftClient {
            client
        }
    }
}