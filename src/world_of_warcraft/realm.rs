use super::{types::realm::{Realm, RealmIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_realm_index(&self) -> Result<RealmIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/realm/index"), "dynamic")
                                .await?
                                .json::<RealmIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_realm(&self, realm_slug: String) -> Result<Realm> {
        let response_result = self.client
                                .send_request(format!("/data/wow/realm/{}", realm_slug), "dynamic")
                                .await?
                                .json::<Realm>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    //TODO: Realm search
}