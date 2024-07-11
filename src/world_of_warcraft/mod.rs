use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
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

#[derive(Clone)]
pub struct WorldOfWarcraftClient {
    pub client: BlizzardAPIClient
}

impl WorldOfWarcraftClient {
    pub fn get (client: BlizzardAPIClient) -> WorldOfWarcraftClient {
        WorldOfWarcraftClient {
            client
        }
    }

    pub async fn get_last_update(&self, href: Href) -> DateTime<Utc> {
        let response  = self.client
                            .send_request_to_href(href)
                            .await;
        let last_modified_string = response.headers()
                                                 .get("Last-Modified")
                                                 .unwrap()
                                                 .to_str()
                                                 .unwrap();
        let date_slice = &last_modified_string[0..last_modified_string.len() - 4];
        let parsed_date = NaiveDateTime::parse_from_str(date_slice, "%a, %d %h %Y %H:%M:%S").unwrap();
        Utc.from_local_datetime(&parsed_date).unwrap()
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