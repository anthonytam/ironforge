use super::{types::mount::{Mount, MountIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_mount_index(&self) -> Result<MountIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/mount/index"), "static")
                                .await?
                                .json::<MountIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_mount(&self, id: u32) -> Result<Mount> {
        let response = self.client
                                .send_request(format!("/data/wow/mount/{}", id), "static")
                                .await?
                                .json::<Mount>()
                                .await?;
        
                                Ok(response)
    }
}