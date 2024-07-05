use super::{types::realm::{Realm, RealmIndex}, world_of_warcraft_client::WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_realm_index(&self) -> RealmIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/realm/index"), "dynamic")
                                .await
                                .json::<RealmIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_realm(&self, realm_slug: String) -> Realm {
        let response_result = self.client
                                .send_request(format!("/data/wow/realm/{}", realm_slug), "dynamic")
                                .await
                                .json::<Realm>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    //TODO: Realm search
}