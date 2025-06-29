use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::creature::{
        CreatureFamily, CreatureFamilyIndex, CreatureSearchParameters, CreatureSearchResponseItem,
        CreatureType,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_creature_family_index(
        &self,
    ) -> Result<CreatureFamilyIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/creature-family/index".to_string(), "static")
            .await
    }

    pub async fn get_creature_family(
        &self,
        id: u32,
    ) -> Result<CreatureFamily, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/creature-family/{id}"), "static")
            .await
    }

    pub async fn get_creature_type_index(
        &self,
    ) -> Result<CreatureFamilyIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/creature-type/index".to_string(), "static")
            .await
    }

    pub async fn get_creature_type(&self, id: u32) -> Result<CreatureType, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/creature-type/{id}"), "static")
            .await
    }

    pub async fn search_creatures(
        &self,
        params: &CreatureSearchParameters,
    ) -> Result<SearchResponse<CreatureSearchResponseItem>, BlizzardAPIClientError> {
        let mut query = vec![];
        if let Some(page) = params._page {
            query.push(format!("_page={page}"));
        }
        if let Some(locale) = &params.locale {
            if let Some(name) = &params.name {
                query.push(format!("name.{locale}={name}"));
            }
        }
        if let Some(orderby) = &params.orderby {
            query.push(format!("orderby={orderby}"));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/data/wow/search/creature{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }
}
