use serde::Deserialize;

use crate::api_client::BlizzardAPIClient;

use super::types::common::Href;


pub struct WorldOfWarcraftClient {
    pub client: BlizzardAPIClient
}

impl WorldOfWarcraftClient {
    pub fn get (client: BlizzardAPIClient) -> WorldOfWarcraftClient {
        WorldOfWarcraftClient {
            client
        }
    }

    pub async fn get_href_data<T: for<'de>Deserialize<'de>>(&self, href: Href) -> T {
        let response_result = self.client
                                .send_request_to_href(href)
                                .await
                                .json::<T>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}