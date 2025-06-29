use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub skill_tiers: Vec<ProfessionSkillTier>,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub professions: Vec<ProfessionSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<ProfessionMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionSkillTierResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub minimum_skill_level: u32,
    pub maximum_skill_level: u32,
    pub categories: Vec<ProfessionSkillTierCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub media: Href,
    pub reagents: Vec<RecipeReagent>,
    pub crafted_item: Option<RecipeCraftedItem>,
    pub alliance_crafted_item: Option<RecipeCraftedItem>,
    pub horde_crafted_item: Option<RecipeCraftedItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<RecipeMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionSkillTier {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionSkillTierCategory {
    pub name: String,
    pub recipes: Vec<ProfessionRecipe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionRecipe {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeReagent {
    pub reagent: RecipeReagentItem,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeReagentItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeCraftedItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}
