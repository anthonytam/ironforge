use crate::api_client::BlizzardAPIClientError;

use super::{types::character::CharacterEquipmentSummary, WorldOfWarcraftClient};
use anyhow::Result;
use reqwest::StatusCode;

impl WorldOfWarcraftClient {
    pub async fn get_character_equipment_summary(
        &self,
        realm_slug: &str,
        character_name: &str,
    ) -> Result<CharacterEquipmentSummary, BlizzardAPIClientError> {
        match self
            .client
            .send_request(
                format!(
                    "/profile/wow/character/{}/{}/equipment",
                    realm_slug, character_name
                ),
                "profile",
            )
            .await
        {
            Ok(response_result) => {
                if response_result.status() == StatusCode::NOT_FOUND {
                    return Err(BlizzardAPIClientError::CharacterNotFound);
                }

                let raw_json = response_result
                    .text()
                    .await
                    .map_err(|e| anyhow::Error::new(e))?;

                match serde_json::from_str::<CharacterEquipmentSummary>(&raw_json) {
                    Ok(character_equipment_summary) => Ok(character_equipment_summary),
                    Err(error) => {
                        #[cfg(test)]
                        println!("Response Body: {}", raw_json);

                        return Err(BlizzardAPIClientError::ParseError(error));
                    }
                }
            }
            Err(error) => Err(BlizzardAPIClientError::BlizzardRequestError(error)),
        }
    }
}
