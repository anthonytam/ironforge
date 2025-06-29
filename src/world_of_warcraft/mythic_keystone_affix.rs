use super::{
    WorldOfWarcraftClient,
    types::mythic_keystone_affix::{
        MythicKeystoneAffix, MythicKeystoneAffixIndex, MythicKeystoneAffixMediaResponse,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_mythic_keystone_affix_index(
        &self,
    ) -> Result<MythicKeystoneAffixIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/keystone-affix/index".to_string(), "static")
            .await
    }

    pub async fn get_mythic_keystone_affix(
        &self,
        affix_id: u32,
    ) -> Result<MythicKeystoneAffix, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/keystone-affix/{affix_id}"), "static")
            .await
    }

    pub async fn get_mythic_keystone_affix_media(
        &self,
        affix_id: u32,
    ) -> Result<MythicKeystoneAffixMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/keystone-affix/{affix_id}"),
                "static",
            )
            .await
    }
}
