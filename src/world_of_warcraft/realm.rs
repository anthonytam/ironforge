use super::{
    WorldOfWarcraftClient,
    types::realm::{Realm, RealmIndex, RealmSearchParameters, RealmSearchResponse},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_realm_index(&self) -> Result<RealmIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/realm/index".to_string(), "dynamic")
            .await
    }

    pub async fn get_realm(&self, realm_slug: String) -> Result<Realm, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/realm/{realm_slug}"), "dynamic")
            .await
    }

    pub async fn search_realm(
        &self,
        params: &RealmSearchParameters,
    ) -> Result<RealmSearchResponse, BlizzardAPIClientError> {
        let mut path = "/data/wow/search/realm".to_string();
        let mut query_params = Vec::new();
        if let Some(page) = params._page {
            query_params.push(format!("_page={page}"));
        }
        if let Some(ref orderby) = params.orderby {
            query_params.push(format!("orderby={orderby}"));
        }
        if let Some(ref timezone) = params.timezone {
            query_params.push(format!("timezone={timezone}"));
        }
        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }
        self.client.request_and_deserialize(path, "dynamic").await
    }
}
