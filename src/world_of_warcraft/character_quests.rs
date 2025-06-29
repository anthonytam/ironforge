use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::character_quests::{CharacterCompletedQuestsResponse, CharacterQuestsResponse},
};

impl WorldOfWarcraftClient {
    pub async fn get_character_quests(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterQuestsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/quests"
                ),
                "profile",
            )
            .await
    }

    pub async fn get_character_completed_quests(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterCompletedQuestsResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/profile/wow/character/{realm_slug}/{character_name}/quests/completed"
                ),
                "profile",
            )
            .await
    }
}
