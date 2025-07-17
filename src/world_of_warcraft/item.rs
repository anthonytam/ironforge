use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::item::{
        Item, ItemClassResponse, ItemMediaResponse, ItemSearchParameters,
        ItemSearchResponseItem, ItemSetResponse, ItemSubclassResponse,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_item(&self, id: u32) -> Result<Item, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/item/{id}"), "static")
            .await
    }

    pub async fn get_item_media(
        &self,
        id: u32,
    ) -> Result<ItemMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/item/{id}"), "static")
            .await
    }

    // TODO: Add item class index
    // pub async fn get_item_class_index(
    //     &self,
    // ) -> Result<ItemClassIndex, BlizzardAPIClientError> {
    //     self.client
    //         .request_and_deserialize("/data/wow/item-class/index".to_string(), "static")
    //         .await
    // }

    pub async fn get_item_class(
        &self,
        class_id: u32,
    ) -> Result<ItemClassResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/item-class/{class_id}"), "static")
            .await
    }

    pub async fn get_item_set(
        &self,
        set_id: u32,
    ) -> Result<ItemSetResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/item-set/{set_id}"), "static")
            .await
    }

    pub async fn get_item_subclass(
        &self,
        class_id: u32,
        subclass_id: u32,
    ) -> Result<ItemSubclassResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/item-class/{class_id}/item-subclass/{subclass_id}"),
                "static",
            )
            .await
    }

    pub async fn search_items(
        &self,
        params: &ItemSearchParameters,
    ) -> Result<SearchResponse<ItemSearchResponseItem>, BlizzardAPIClientError> {
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
        let path = format!("/data/wow/search/item{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }
}

#[cfg(test)]
mod item_tests {
    use crate::world_of_warcraft::{test_utils::test_utils::{create_test_client, print_error}, types::item::{ItemSearchParameters, ItemSearchResponseItem}};

    #[tokio::test]
    async fn test_item_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Item Functions ===");
        
        let item_search = client.search_items(&ItemSearchParameters {
            _page: Some(1),
            locale: Some("en_US".to_string()),
            name: Some("Thunderfury".to_string()),
            orderby: Some("id".to_string()),
        }).await;
        print_error(&item_search, "search_items");
        
        let item_search = item_search.unwrap();
        let item_id = client.get_href_data::<ItemSearchResponseItem>(&item_search.results.first().unwrap().key).await.unwrap().data.id;
        let result = client.get_item(item_id).await;
        print_error(&result, "get_item");

        let result = client.get_item_media(item_id).await;
        print_error(&result, "get_item_media");
    }
}   