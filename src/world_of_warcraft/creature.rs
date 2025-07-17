use super::{
    WorldOfWarcraftClient,
    types::common::SearchResponse,
    types::creature::{
        CreatureFamily, CreatureFamilyIndex, CreatureSearchParameters, CreatureSearchResponseItem,
        CreatureType, CreatureTypesIndex,
    },
};
use crate::{api_client::{ApiRequestHelper, BlizzardAPIClientError}, world_of_warcraft::types::creature::{Creature, CreatureDisplay, CreatureFamilyMedia}};

impl WorldOfWarcraftClient {
    pub async fn get_creature(&self, id: u32) -> Result<Creature, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/creature/{id}"), "static")
            .await
    }

    pub async fn get_creature_family_index(
        &self,
    ) -> Result<CreatureFamilyIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/creature-family/index".to_string(), "static")
            .await
    }

    pub async fn get_creature_display_media(&self, id: u32) -> Result<CreatureDisplay, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/creature-display/{id}"), "static")
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

    pub async fn get_creature_family_media(&self, id: u32) -> Result<CreatureFamilyMedia, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(format!("/data/wow/media/creature-family/{id}"), "static")
            .await
    }

    pub async fn get_creature_type_index(
        &self,
    ) -> Result<CreatureTypesIndex, BlizzardAPIClientError> {
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

#[cfg(test)]
mod creature_tests {
    use crate::world_of_warcraft::{test_utils::test_utils::{create_test_client, print_error}, types::creature::{CreatureSearchParameters, CreatureSearchResponseItem}};

    #[tokio::test]
    async fn test_creature_functions() {
        let client = create_test_client().await;
        
        println!("\n=== Testing Creature Functions ===");
        
        let creature_search_params = CreatureSearchParameters {
            _page: Some(1),
            locale: Some("en_US".to_string()),
            name: Some("Dragon".to_string()),
            orderby: Some("id".to_string()),
        };
        let creature_search_response = client.search_creatures(&creature_search_params).await;
        print_error(&creature_search_response, "search_creatures");

        let creature_search_response = creature_search_response.unwrap();
        let creature_id = client.get_href_data::<CreatureSearchResponseItem>(&creature_search_response.results.first().unwrap().key).await.unwrap().data.id;
        let creature = client.get_creature(creature_id).await;
        print_error(&creature, "get_creature");

        let creature = creature.unwrap();
        let creature_display_media = client.get_creature_display_media(creature.creature_displays.first().unwrap().id).await;
        print_error(&creature_display_media, "get_creature_display_media");

        let creature_family_index = client.get_creature_family_index().await;
        print_error(&creature_family_index, "get_creature_family_index");

        let creature_family_index = creature_family_index.unwrap();
        let creature_family = creature_family_index.creature_families.first().unwrap();
        let creature_family = client.get_creature_family(creature_family.id).await;
        print_error(&creature_family, "get_creature_family");

        let creature_family = creature_family.unwrap();
        let creature_family_media = client.get_creature_family_media(creature_family.id).await;
        print_error(&creature_family_media, "get_creature_family_media");

        let creature_type_index = client.get_creature_type_index().await;
        print_error(&creature_type_index, "get_creature_type_index");

        let creature_type_index = creature_type_index.unwrap();
        let creature_type = creature_type_index.creature_types.first().unwrap();
        let creature_type = client.get_creature_type(creature_type.id).await;
        print_error(&creature_type, "get_creature_type");
    }
}   