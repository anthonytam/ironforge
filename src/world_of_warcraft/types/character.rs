use serde::{Deserialize, Serialize};

use super::common::{Links, Key, Realm, Media, Color, Link};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipmentSummary {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: Character,
    pub equipped_items: Vec<EquippedItem>,
    pub equipped_item_sets: Option<Vec<EquippedItemSet>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub key: Key,
    pub name: String,
    pub id: i64,
    pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub item: Item,
    pub slot: Slot,
    pub quantity: i64,
    pub context: Option<i64>,
    #[serde(default)]
    pub bonus_list: Vec<i64>,
    pub quality: Quality,
    pub name: String,
    pub modified_appearance_id: Option<i64>,
    pub media: Media,
    pub item_class: ItemClass,
    pub item_subclass: ItemSubclass,
    pub inventory_type: InventoryType,
    pub binding: Binding,
    pub armor: Option<Armor>,
    #[serde(default)]
    pub stats: Vec<Stat>,
    pub sell_price: Option<SellPrice>,
    pub requirements: Option<Requirements>,
    pub set: Option<Set>,
    pub level: Level,
    pub durability: Option<Durability>,
    pub name_description: Option<NameDescription>,
    pub is_subclass_hidden: Option<bool>,
    pub transmog: Option<Transmog>,
    #[serde(default)]
    pub spells: Vec<Spell>,
    pub unique_equipped: Option<String>,
    pub weapon: Option<Weapon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub key: Key,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemClass {
    pub key: Key,
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSubclass {
    pub key: Key,
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binding {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    pub value: i64,
    pub display: Display,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    pub display_string: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    #[serde(rename = "type")]
    pub type_field: Type,
    pub value: i64,
    pub display: Display,
    pub is_equip_bonus: Option<bool>,
    pub is_negated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellPrice {
    pub value: i64,
    pub display_strings: DisplayStrings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayStrings {
    pub header: String,
    pub gold: String,
    pub silver: String,
    pub copper: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub level: Option<Level>,
    pub playable_classes: Option<PlayableClasses>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub value: Option<i32>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClasses {
    pub links: Vec<Link>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    pub item_set: ItemSet,
    pub items: Vec<ItemEquipState>,
    pub effects: Vec<Effect>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSet {
    pub key: Key,
    pub name: String,
    pub id: i64,
}

//TODO: Come up with the right type name
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemEquipState {
    pub item: Item3,
    pub is_equipped: Option<bool>,
}

//TODO: Come up with the right type name
#[derive(Debug, Serialize, Deserialize)]
pub struct Item3 {
    pub key: Key,
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    pub display_string: String,
    pub required_count: i64,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Durability {
    pub value: i64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameDescription {
    pub display_string: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transmog {
    pub item: Item3,
    pub display_string: String,
    pub item_modified_appearance_id: i64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Spell {
    pub spell: Spell2,
    pub description: String,
}

//TODO: Come up with the right type name
#[derive(Debug, Serialize, Deserialize)]
pub struct Spell2 {
    pub key: Key,
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub damage: Damage,
    pub attack_speed: AttackSpeed,
    pub dps: Dps,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damage {
    pub min_value: i64,
    pub max_value: i64,
    pub display_string: String,
    pub damage_class: DamageClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackSpeed {
    pub value: i64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dps {
    pub value: f64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItemSet {
    pub item_set: ItemSet,
    pub items: Vec<ItemEquipState>,
    pub effects: Vec<Effect>,
    pub display_string: String,
}