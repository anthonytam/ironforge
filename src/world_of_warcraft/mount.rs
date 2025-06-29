use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::mount::{Mount, MountIndex, MountSearchParameters, MountSearchResponseItem},
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_mount_index(&self) -> Result<MountIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/mount/index".to_string(), "static")
            .await
    }

    pub async fn get_mount(&self, id: u32) -> Result<Mount, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/mount/{id}"), "static")
            .await
    }

    pub async fn search_mounts(
        &self,
        params: &MountSearchParameters,
    ) -> Result<SearchResponse<MountSearchResponseItem>, BlizzardAPIClientError> {
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
        let path = format!("/data/wow/search/mount{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }
}
