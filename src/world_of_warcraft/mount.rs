use super::{types::mount::{Mount, MountIndex}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_mount_index(&self) -> MountIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/mount/index"), "static")
                                .await
                                .json::<MountIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }

    pub async fn get_mount(&self, id: u32) -> Mount {
        let response_result = self.client
                                .send_request(format!("/data/wow/mount/{}", id), "static")
                                .await
                                .json::<Mount>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a repsonse. {:?}", e)
        }
    }
}