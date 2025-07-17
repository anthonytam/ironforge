use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

use super::{WorldOfWarcraftClient, types::common::SearchResponse, types::journal::*};

impl WorldOfWarcraftClient {
    pub async fn get_journal_encounter(
        &self,
        journal_encounter_id: u32,
    ) -> Result<JournalEncounterResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/journal-encounter/{journal_encounter_id}"),
                "static",
            )
            .await
    }

    pub async fn get_journal_encounter_index(
        &self,
    ) -> Result<JournalEncounterIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/journal-encounter/index".to_string(), "static")
            .await
    }

    pub async fn journal_encounter_search(
        &self,
        options: &JournalEncounterSearchParameters,
    ) -> Result<SearchResponse<JournalEncounterSearchResponseItem>, BlizzardAPIClientError> {
        let mut params = vec![];

        if let Some(page) = options._page {
            params.push(format!("_page={page}"));
        }

        if let Some(instance_name) = &options.instance_name {
            let locale = options.locale.as_deref().unwrap_or("en_US");
            params.push(format!("instance.name.{locale}={instance_name}"));
        }

        if let Some(orderby) = &options.orderby {
            params.push(format!("orderby={orderby}"));
        }

        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };

        let path = format!("/data/wow/search/journal-encounter{query_string}");
        self.client.request_and_deserialize(path, "static").await
    }

    pub async fn get_journal_expansion(
        &self,
        journal_expansion_id: u32,
    ) -> Result<JournalExpansionResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/journal-expansion/{journal_expansion_id}"),
                "static",
            )
            .await
    }

    pub async fn get_journal_expansion_index(
        &self,
    ) -> Result<JournalExpansionIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/journal-expansion/index".to_string(), "static")
            .await
    }

    pub async fn get_journal_instance(
        &self,
        journal_instance_id: u32,
    ) -> Result<JournalInstanceResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/journal-instance/{journal_instance_id}"),
                "static",
            )
            .await
    }

    pub async fn get_journal_instance_index(
        &self,
    ) -> Result<JournalInstanceIndexResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/journal-instance/index".to_string(), "static")
            .await
    }

    pub async fn get_journal_instance_media(
        &self,
        journal_instance_id: u32,
    ) -> Result<JournalInstanceMediaResponse, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/media/journal-instance/{journal_instance_id}"),
                "static",
            )
            .await
    }
}
