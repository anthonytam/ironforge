use super::{types::covenant::{Conduit, ConduitIndex, Covenant, CovenantIndex, CovenantMedia, Soulbind, SoulbindIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_covenant_index(&self) -> CovenantIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/index"), "static")
                                .await
                                .json::<CovenantIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_covenant(&self, id: u32) -> Covenant {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/{}", id), "static")
                                .await
                                .json::<Covenant>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_covenant_media(&self, id: u32) -> CovenantMedia {
        let response_result = self.client
                                .send_request(format!("/data/wow/media/covenant/{}", id), "static")
                                .await
                                .json::<CovenantMedia>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_soulbind_index(&self) -> SoulbindIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/soulbind/index"), "static")
                                .await
                                .json::<SoulbindIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_soulbind(&self, id: u32) -> Soulbind {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/soulbind/{}", id), "static")
                                .await
                                .json::<Soulbind>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_conduit_index(&self) -> ConduitIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/conduit/index"), "static")
                                .await
                                .json::<ConduitIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_conduit(&self) -> Conduit {
        let response_result = self.client
                                .send_request(format!("/data/wow/covenant/conduit/index"), "static")
                                .await
                                .json::<Conduit>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
    
}