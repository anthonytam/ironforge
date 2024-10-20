use serde::{Deserialize, Serialize};

use super::common::Links;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipmentSummary {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: Character,
    pub equipped_items: Vec<EquippedItem>,
    //pub name: String,
    //pub description: String,
    //pub points: u32,
    //pub is_account_wide: bool,
    pub next_achievement: Option<u32>,
    //pub display_order: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub level: Level,
    pub slot: Slot,
    pub inventory_type: InventoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub value: i32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryType {
    #[serde(rename = "type")]
    pub enumtype: String,
    pub name: String,
}