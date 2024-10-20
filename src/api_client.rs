use std::{sync::Arc, time::{Duration, SystemTime}};

use reqwest::Response;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{types::BlizzardAccessTokenResponse, world_of_warcraft::types::common::Href};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Region {
    US,
    EU,
    KR,
    TW,
    CN,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Locale {
    en_US,
    es_MX,
    pt_BR,
    en_GB,
    es_ES,
    fr_FR,
    ru_RU,
    de_DE,
    pt_PT,
    it_IT,
    ko_KR,
    zh_TW,
    zh_CN,
}

#[derive(Clone)]
pub struct BlizzardAPIClient {
    reqwest_client: reqwest::Client,
    region: Region,
    locale: Locale,
    client_id: String,
    client_secret: String,
    access_token: Arc<Mutex<Option<String>>>,
    token_expiration: Arc<Mutex<SystemTime>>,
}

impl BlizzardAPIClient {
    pub fn new( client_id: String, client_secret: String, region: Region, locale: Locale) -> BlizzardAPIClient {
        BlizzardAPIClient {
            reqwest_client: reqwest::ClientBuilder::new().build().unwrap(),
            region: region,
            locale: locale,
            client_id: client_id,
            client_secret: client_secret,
            access_token: Arc::new(Mutex::new(None)),
            token_expiration: Arc::new(Mutex::new(SystemTime::now())),
        }
    }

    async fn get_new_access_token (&self) {
        let new_token_response = self.reqwest_client
                                                .post(self.get_token_url())
                                                .form(&[("grant_type", "client_credentials")])
                                                .basic_auth(&self.client_id, Some(&self.client_secret))
                                                .send()
                                                .await;
        match new_token_response {
            Ok(new_token) => {
                match new_token.json::<BlizzardAccessTokenResponse>().await {
                    Ok(blizzard_access_token_response) => {
                        if let Ok(mut expiration) = self.token_expiration.try_lock() {
                            *expiration = SystemTime::now().checked_add(Duration::from_secs(blizzard_access_token_response.expires_in)).unwrap()
                        }
                        if let Ok(mut access_token) = self.access_token.try_lock() {
                            *access_token = Some(blizzard_access_token_response.access_token);
                        }
                    }
                    Err(e) => panic!("Failed to deserialize access token request {:?}", e)
                }
            },
            Err(e) => panic!("Failed to get a new access token {:?}", e)
        }
    }

    fn is_access_token_valid (&self) -> bool {
        match self.access_token.try_lock() {
            Ok(token) => match token.as_ref() {
                None => false,
                Some(_) => {
                    self.is_token_expired()
                }
            }
            Err(e) => panic!("{}",e.to_string())
        }
    }

    fn is_token_expired (&self) -> bool {
        match self.token_expiration.try_lock() {
            Ok(expiration_time) => expiration_time.le(&SystemTime::now()),
            Err(e) => panic!("{}",e.to_string())
        }
    }

    pub async fn send_request(&self, url_path: String, namespace: &str) -> Response {
        if !self.is_access_token_valid() {
            self.get_new_access_token().await;
        }
        let locked_token = self.access_token.try_lock().unwrap();
        let access_token = locked_token.as_ref().unwrap();

        let response_result = self.reqwest_client
                       .get(format!("{}{}?locale={}", self.get_api_url(), url_path, self.locale))
                       .header("Battlenet-Namespace", format!("{}-{}", namespace, self.region))
                       .bearer_auth(access_token)
                       .send()
                       .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Bad response from Blizzard. {:?}", e)
        }
    }

    pub async fn send_request_to_href(&self, href: Href) -> Response {
        if !self.is_access_token_valid() {
            self.get_new_access_token().await;
        }
        let locked_token = self.access_token.try_lock().unwrap();
        let access_token = locked_token.as_ref().unwrap();
        let response_result = self.reqwest_client
                       .get(format!("{}&locale={}", href.href, self.locale))
                       .bearer_auth(access_token)
                       .send()
                       .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Bad response from Blizzard. {:?}", e)
        }
    }

    fn get_api_url(&self) -> String {
        match self.region {
            Region::US | Region::EU | Region::KR | Region::TW => "https://gateway.battlenet.com.cn".to_string(),
            Region::CN => "https://gateway.battlenet.com.cn".to_string(),
        }
    }

    fn get_token_url(&self) -> String {
        match self.region {
            Region::US | Region::EU | Region::KR | Region::TW => "https://gateway.battlenet.com.cn".to_string(),
            Region::CN => "https://gateway.battlenet.com.cn".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use tokio;

    use crate::world_of_warcraft::WorldOfWarcraftClient;

    use super::*;

    // Basic test ensures a Blizzard API client can be created, a WoWClient can be created, and an API call can be made.
    #[tokio::test]
    async fn it_works() {
        dotenv().ok();

        let blizzard_client = BlizzardAPIClient::new(env::var("BLIZZARD_CLIENT_ID").unwrap(),
                                                                    env::var("BLIZZARD_CLIENT_SECRET").unwrap(),
                                                                    Region::US,
                                                                    Locale::en_US);

        let wow_client = WorldOfWarcraftClient::get(blizzard_client);
        let _achievements_index = wow_client.get_achievements_index().await;
    }
}
