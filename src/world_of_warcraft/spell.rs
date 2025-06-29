use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::spell::{
        SpellMediaResponse, SpellResponse, SpellSearchParameters, SpellSearchResponseItem,
    },
};

impl WorldOfWarcraftClient {
    pub async fn get_spell(&self, spell_id: u32) -> Result<SpellResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/spell/{spell_id}"), "static")
            .await
    }

    pub async fn get_spell_media(
        &self,
        spell_id: u32,
    ) -> Result<SpellMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/spell/{spell_id}"), "static")
            .await
    }

    pub async fn search_spells(
        &self,
        params: &SpellSearchParameters,
    ) -> Result<SearchResponse<SpellSearchResponseItem>, BlizzardAPIClientError> {
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
        let path = format!("/data/wow/search/spell{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }
}
