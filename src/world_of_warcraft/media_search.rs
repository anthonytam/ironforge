use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::media_search::{MediaSearchParameters, MediaSearchResponse},
};

impl WorldOfWarcraftClient {
    pub async fn search_media(
        &self,
        params: &MediaSearchParameters,
    ) -> Result<MediaSearchResponse, BlizzardAPIClientError> {
        let mut path = "/data/wow/search/media".to_string();
        let mut query_params = Vec::new();

        if let Some(page) = params._page {
            query_params.push(format!("_page={page}"));
        }

        if let Some(ref orderby) = params.orderby {
            query_params.push(format!("orderby={orderby}"));
        }

        if let Some(ref tags) = params.tags {
            query_params.push(format!("tags={tags}"));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.request_and_deserialize(path, "static").await
    }
}
