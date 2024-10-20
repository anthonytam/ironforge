use super::{types::connected_realm::{ConnectedRealm, ConnectedRealmsIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_connected_realms_index(&self) -> Result<ConnectedRealmsIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/connected-realm/index"), "dynamic")
                                .await?
                                .json::<ConnectedRealmsIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_connected_realm(&self, id: u32) -> Result<ConnectedRealm> {
        let response = self.client
                                .send_request(format!("/data/wow/connected-realm/{}", id), "dynamic")
                                .await?
                                .json::<ConnectedRealm>()
                                .await?;
        
                                Ok(response)
    }

    //TODO: Implement Connected Realms Search
}