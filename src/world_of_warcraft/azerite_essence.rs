use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::azerite_essence::{
        AzeriteEssenceDetail, AzeriteEssenceIndex, AzeriteEssenceMedia,
        AzeriteEssenceSearchParameters, AzeriteEssenceSearchResponseItem,
    },
    types::common::SearchResponse,
};

impl WorldOfWarcraftClient {
    pub async fn get_azerite_essence_index(
        &self,
    ) -> Result<AzeriteEssenceIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/azerite-essence/index".to_string(), "static")
            .await
    }

    pub async fn get_azerite_essence(
        &self,
        essence_id: u32,
    ) -> Result<AzeriteEssenceDetail, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/azerite-essence/{essence_id}"),
                "static",
            )
            .await
    }

    pub async fn get_azerite_essence_media(
        &self,
        essence_id: u32,
    ) -> Result<AzeriteEssenceMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/azerite-essence/{essence_id}/media"),
                "static",
            )
            .await
    }

    pub async fn search_azerite_essences(
        &self,
        params: &AzeriteEssenceSearchParameters,
    ) -> Result<SearchResponse<AzeriteEssenceSearchResponseItem>, BlizzardAPIClientError> {
        let mut query = vec![];
        if let Some(page) = params._page {
            query.push(format!("_page={page}"));
        }
        if let Some(orderby) = &params.orderby {
            query.push(format!("orderby={orderby}"));
        }
        if let Some(allowed_specializations_id) = params.allowed_specializations_id {
            query.push(format!(
                "allowed_specializations.id={allowed_specializations_id}"
            ));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/data/wow/search/azerite-essence{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }
}
