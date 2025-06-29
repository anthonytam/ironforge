use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub categories: Href,
    pub slot_types: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingCategoryIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub categories: Vec<ModifiedCraftingCategorySummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingCategorySummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingReagentSlotTypeIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub slot_types: Vec<ModifiedCraftingSlotTypeSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingSlotTypeSummary {
    pub key: Href,
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingCategory {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedCraftingReagentSlotType {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub description: String,
    pub compatible_categories: Vec<CompatibleCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibleCategory {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
