use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::profession::{
        ProfessionIndexResponse, ProfessionMediaResponse, ProfessionResponse,
        ProfessionSkillTierResponse, RecipeMediaResponse, RecipeResponse,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_profession(
        &self,
        profession_id: u32,
    ) -> Result<ProfessionResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/profession/{profession_id}"), "static")
            .await
    }

    pub async fn get_profession_index(
        &self,
    ) -> Result<ProfessionIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/profession/index".to_string(), "static")
            .await
    }

    pub async fn get_profession_media(
        &self,
        profession_id: u32,
    ) -> Result<ProfessionMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/profession/{profession_id}"),
                "static",
            )
            .await
    }

    pub async fn get_profession_skill_tier(
        &self,
        profession_id: u32,
        skill_tier_id: u32,
    ) -> Result<ProfessionSkillTierResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/profession/{profession_id}/skill-tier/{skill_tier_id}"
                ),
                "static",
            )
            .await
    }

    pub async fn get_recipe(
        &self,
        recipe_id: u32,
    ) -> Result<RecipeResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/recipe/{recipe_id}"), "static")
            .await
    }

    pub async fn get_recipe_media(
        &self,
        recipe_id: u32,
    ) -> Result<RecipeMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/recipe/{recipe_id}"), "static")
            .await
    }
}
