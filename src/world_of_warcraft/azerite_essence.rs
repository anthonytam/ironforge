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
        essence_media_id: u32,
    ) -> Result<AzeriteEssenceMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/azerite-essence/{essence_media_id}"),
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

#[cfg(test)]
mod azerite_essence_tests {
    use crate::world_of_warcraft::{test_utils::test_utils::{create_test_client, print_error}, types::azerite_essence::AzeriteEssenceSearchParameters};

    #[tokio::test]
    async fn test_azerite_essence_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Azerite Essence Functions ===");
        
        let azerite_essence_index = client.get_azerite_essence_index().await;
        print_error(&azerite_essence_index, "get_azerite_essence_index");
        
        let azerite_essence_id = azerite_essence_index.unwrap().azerite_essences.first().unwrap().id;
        let azerite_essence = client.get_azerite_essence(azerite_essence_id).await;
        print_error(&azerite_essence, "get_azerite_essence");
        
        let azerite_essence_media_id = azerite_essence.unwrap().media.id;
        let azerite_essence_media = client.get_azerite_essence_media(azerite_essence_media_id).await;
        print_error(&azerite_essence_media, "get_azerite_essence_media");
        
        let search_params = AzeriteEssenceSearchParameters {
            _page: Some(1),
            orderby: Some("name".to_string()),
            allowed_specializations_id: Some(65),
        };
        let search_result = client.search_azerite_essences(&search_params).await;
        print_error(&search_result, "search_azerite_essences");
    }
}