use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::playable_class::{
        PlayableClassIndexResponse, PlayableClassMediaResponse, PlayableClassResponse,
        PvpTalentSlotsResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_playable_class(
        &self,
        class_id: u32,
    ) -> Result<PlayableClassResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/playable-class/{class_id}"), "static")
            .await
    }

    pub async fn get_playable_class_index(
        &self,
    ) -> Result<PlayableClassIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/playable-class/index".to_string(), "static")
            .await
    }

    pub async fn get_playable_class_media(
        &self,
        class_id: u32,
    ) -> Result<PlayableClassMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/playable-class/{class_id}"),
                "static",
            )
            .await
    }

    pub async fn get_pvp_talent_slots(
        &self,
        class_id: u32,
    ) -> Result<PvpTalentSlotsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/playable-class/{class_id}/pvp-talent-slots"),
                "static",
            )
            .await
    }
}
