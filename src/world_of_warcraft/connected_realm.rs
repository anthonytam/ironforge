use super::{types::connected_realm::{ConnectedRealm, ConnectedRealmsIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_connected_realms_index(&self) -> Result<ConnectedRealmsIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/connected-realm/index"), "dynamic")
                                .await?
                                .json::<ConnectedRealmsIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_connected_realm(&self, id: u32) -> Result<ConnectedRealm> {
        let response_result = self.client
                                .send_request(format!("/data/wow/connected-realm/{}", id), "dynamic")
                                .await?
                                .json::<ConnectedRealm>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    //TODO: Implement Connected Realms Search
}