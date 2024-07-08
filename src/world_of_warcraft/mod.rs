use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use types::common::Href;

use crate::api_client::BlizzardAPIClient;

pub mod types;

pub mod achievements;
pub mod auction_house;
pub mod azerite_essence;
pub mod connected_realm;
pub mod covenant;
pub mod creature;
pub mod guild_crest;
pub mod heirloom;
pub mod mount;
pub mod realm;
pub mod region;
pub mod title;
pub mod toy;
pub mod wow_token;

pub struct WorldOfWarcraftClient {
    pub client: BlizzardAPIClient
}

impl WorldOfWarcraftClient {
    pub fn get (client: BlizzardAPIClient) -> WorldOfWarcraftClient {
        WorldOfWarcraftClient {
            client
        }
    }

    pub async fn get_last_update(&self, href: Href) -> DateTime<FixedOffset> {
        let response  = self.client
                            .send_request_to_href(href)
                            .await;
        let last_modified_string = response.headers()
                                                 .get("last-modified")
                                                 .unwrap()
                                                 .to_str()
                                                 .unwrap();
        DateTime::parse_from_str(last_modified_string, "%a, %e %b %Y &k:%M:%S %Z").unwrap()
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