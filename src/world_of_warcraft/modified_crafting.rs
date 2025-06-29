use super::{
    WorldOfWarcraftClient,
    types::modified_crafting::{
        ModifiedCraftingCategory, ModifiedCraftingCategoryIndex, ModifiedCraftingIndex,
        ModifiedCraftingReagentSlotType, ModifiedCraftingReagentSlotTypeIndex,
    },
};
use crate::api_client::{ApiRequestHelper, BlizzardAPIClientError};

impl WorldOfWarcraftClient {
    pub async fn get_modified_crafting_index(
        &self,
    ) -> Result<ModifiedCraftingIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize("/data/wow/modified-crafting/index".to_string(), "static")
            .await
    }

    pub async fn get_modified_crafting_category_index(
        &self,
    ) -> Result<ModifiedCraftingCategoryIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/modified-crafting/category/index".to_string(),
                "static",
            )
            .await
    }

    pub async fn get_modified_crafting_reagent_slot_type_index(
        &self,
    ) -> Result<ModifiedCraftingReagentSlotTypeIndex, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                "/data/wow/modified-crafting/reagent-slot-type/index".to_string(),
                "static",
            )
            .await
    }

    pub async fn get_modified_crafting_category(
        &self,
        category_id: u32,
    ) -> Result<ModifiedCraftingCategory, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!("/data/wow/modified-crafting/category/{category_id}"),
                "static",
            )
            .await
    }

    pub async fn get_modified_crafting_reagent_slot_type(
        &self,
        slot_type_id: u32,
    ) -> Result<ModifiedCraftingReagentSlotType, BlizzardAPIClientError> {
        self.client
            .request_and_deserialize(
                format!(
                    "/data/wow/modified-crafting/reagent-slot-type/{slot_type_id}"
                ),
                "static",
            )
            .await
    }
}
