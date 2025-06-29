use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::item::{
        Item, ItemClassResponse, ItemIndex, ItemMediaResponse, ItemSearchParameters,
        ItemSearchResponseItem, ItemSetResponse, ItemSubclassResponse,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_item_index(&self) -> Result<ItemIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/item/index".to_string(), "static")
            .await
    }

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
