use super::{types::realm::{Realm, RealmIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_realm_index(&self) -> RealmIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/realm/index"), "dynamic")
                                .await
                                .json::<RealmIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
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
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    //TODO: Realm search
}