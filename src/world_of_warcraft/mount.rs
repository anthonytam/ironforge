use super::{types::mount::{Mount, MountIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_mount_index(&self) -> Result<MountIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/mount/index"), "static")
                                .await?
                                .json::<MountIndex>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_mount(&self, id: u32) -> Result<Mount> {
        let response_result = self.client
                                .send_request(format!("/data/wow/mount/{}", id), "static")
                                .await?
                                .json::<Mount>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }
}