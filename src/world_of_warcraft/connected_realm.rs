use super::{types::connected_realm::{ConnectedRealm, ConnectedRealmsIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_connected_realms_index(&self) -> ConnectedRealmsIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/connected-realm/index"), "dynamic")
                                .await
                                .json::<ConnectedRealmsIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_connected_realm(&self, id: u32) -> ConnectedRealm {
        let response_result = self.client
                                .send_request(format!("/data/wow/connected-realm/{}", id), "dynamic")
                                .await
                                .json::<ConnectedRealm>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    //TODO: Implement Connected Realms Search
}