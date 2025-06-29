use super::{
    WorldOfWarcraftClient,
    types::connected_realm::{ConnectedRealm, ConnectedRealmsIndex},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_connected_realms_index(
        &self,
    ) -> Result<ConnectedRealmsIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/connected-realm/index".to_string(), "dynamic")
            .await
    }

    pub async fn get_connected_realm(
        &self,
        id: u32,
    ) -> Result<ConnectedRealm, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/connected-realm/{id}"), "dynamic")
            .await
    }
}
