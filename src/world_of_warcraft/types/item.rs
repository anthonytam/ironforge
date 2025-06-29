use serde::{Deserialize, Serialize};

use super::common::{Color, Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub items: Vec<ItemSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub quality: TypeNode,
    pub level: u32,
    pub required_level: u32,
    pub media: ItemMedia,
    pub item_class: ItemClass,
    pub item_subclass: ItemSubclass,
    pub inventory_type: TypeNode,
    pub purchase_price: u32,
    pub sell_price: u32,
    pub max_count: u32,
    pub is_equippable: bool,
    pub is_stackable: bool,
    pub preview_item: Option<PreviewItem>,
    pub purchase_quantity: u32,
    pub appearances: Option<Vec<ItemAppearance>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSubclass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAppearance {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewItem {
    pub item: ItemReference,
    pub context: u32,
    pub bonus_list: Vec<u32>,
    pub quality: TypeNode,
    pub name: String,
    pub media: ItemMedia,
    pub item_class: ItemClass,
    pub item_subclass: ItemSubclass,
    pub inventory_type: TypeNode,
    pub binding: Option<TypeNode>,
    pub unique_equipped: Option<String>,
    pub weapon: Option<Weapon>,
    pub stats: Vec<ItemStat>,
    pub spells: Vec<ItemSpell>,
    pub requirements: ItemRequirements,
    pub level: ItemLevel,
    pub durability: Option<ItemDurability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemReference {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub damage: WeaponDamage,
    pub attack_speed: WeaponAttackSpeed,
    pub dps: WeaponDps,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponDamage {
    pub min_value: u32,
    pub max_value: u32,
    pub display_string: String,
    pub damage_class: TypeNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponAttackSpeed {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponDps {
    pub value: f64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStat {
    #[serde(rename = "type")]
    pub stat_type: TypeNode,
    pub value: i32,
    pub is_negated: Option<bool>,
    pub display: ItemStatDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatDisplay {
    pub display_string: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSpell {
    pub spell: SpellReference,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellReference {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRequirements {
    pub level: ItemRequirement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRequirement {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemLevel {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDurability {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemMediaResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<ItemMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemClassResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub class_id: u32,
    pub name: String,
    pub item_subclasses: Vec<ItemSubclassSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSubclassSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSetResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub items: Vec<ItemSetItem>,
    pub effects: Vec<ItemSetEffect>,
    pub is_effect_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSetItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSetEffect {
    pub display_string: String,
    pub required_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSubclassResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub class_id: u32,
    pub subclass_id: u32,
    pub display_name: String,
    pub hide_subclass_in_tooltips: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchParameters {
    pub _page: Option<u32>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub orderby: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchResponseItem {
    pub key: Href,
    pub data: ItemSearchData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchData {
    pub id: u32,
    pub inventory_type: Option<String>,
    pub is_equippable: bool,
    pub is_stackable: bool,
    pub item_class: Option<ItemSearchClass>,
    pub item_subclass: Option<ItemSearchSubclass>,
    pub level: u32,
    pub max_count: u32,
    pub media: Option<ItemSearchMedia>,
    pub name: Option<std::collections::HashMap<String, String>>,
    pub purchase_price: u32,
    pub purchase_quantity: u32,
    pub quality: Option<String>,
    pub required_level: u32,
    pub sell_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchClass {
    pub id: u32,
    pub name: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchSubclass {
    pub id: u32,
    pub name: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSearchMedia {
    pub id: u32,
}
