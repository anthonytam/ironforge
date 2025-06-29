use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipmentSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterEquipmentCharacter,
    pub equipped_items: Vec<CharacterEquippedItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipmentCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterEquipmentRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipmentRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquippedItem {
    pub item: CharacterItem,
    pub slot: CharacterItemSlot,
    pub quantity: u32,
    pub context: u32,
    pub bonus_list: Option<Vec<u32>>,
    pub quality: CharacterItemQuality,
    pub name: String,
    pub modified_appearance_id: Option<u32>,
    pub azerite_details: Option<CharacterItemAzeriteDetails>,
    pub name_description: Option<String>,
    pub media: Href,
    pub item_class: CharacterItemClass,
    pub item_subclass: CharacterItemSubclass,
    pub inventory_type: CharacterInventoryType,
    pub binding: CharacterItemBinding,
    pub armor: Option<CharacterItemArmor>,
    pub stats: Option<Vec<CharacterItemStat>>,
    pub spells: Option<Vec<CharacterItemSpell>>,
    pub sell_price: Option<CharacterItemSellPrice>,
    pub requirements: Option<CharacterItemRequirements>,
    pub level: CharacterItemLevel,
    pub transmog: Option<CharacterItemTransmog>,
    pub durability: Option<CharacterItemDurability>,
    pub unique_equipped: Option<String>,
    pub set: Option<CharacterItemSet>,
    pub socket: Option<CharacterItemSocket>,
    pub enchantments: Option<Vec<CharacterItemEnchantment>>,
    pub weapon: Option<CharacterItemWeapon>,
    pub preview_item: Option<Href>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSlot {
    #[serde(rename = "type")]
    pub slot_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemQuality {
    #[serde(rename = "type")]
    pub quality_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemAzeriteDetails {
    pub selected_powers: Vec<CharacterItemAzeritePower>,
    pub selected_powers_string: String,
    pub percentage_to_next_level: f32,
    pub selected_essences: Option<Vec<CharacterItemAzeriteEssence>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemAzeritePower {
    pub id: u32,
    pub tier: u32,
    pub spell_tooltip: Option<CharacterItemSpellTooltip>,
    pub is_display_hidden: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemAzeriteEssence {
    pub slot: u32,
    pub rank: u32,
    pub main_spell_tooltip: Option<CharacterItemSpellTooltip>,
    pub passive_spell_tooltip: Option<CharacterItemSpellTooltip>,
    pub essence: CharacterItemAzeriteEssenceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemAzeriteEssenceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSubclass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterInventoryType {
    #[serde(rename = "type")]
    pub inventory_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemBinding {
    #[serde(rename = "type")]
    pub binding_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemArmor {
    pub value: u32,
    pub display: CharacterItemArmorDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemArmorDisplay {
    pub display_string: String,
    pub color: CharacterItemColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemStat {
    #[serde(rename = "type")]
    pub stat_type: CharacterItemStatType,
    pub value: i32,
    pub display: CharacterItemStatDisplay,
    pub is_negated: Option<bool>,
    pub is_equip_bonus: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemStatType {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemStatDisplay {
    pub display_string: String,
    pub color: CharacterItemColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSpell {
    pub spell: CharacterItemSpellInfo,
    pub description: String,
    pub display_color: Option<CharacterItemColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSpellInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSellPrice {
    pub value: u32,
    pub display_strings: CharacterItemSellPriceDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSellPriceDisplay {
    pub header: String,
    pub gold: String,
    pub silver: String,
    pub copper: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemRequirements {
    pub level: Option<CharacterItemLevelRequirement>,
    pub playable_classes: Option<CharacterItemClassRequirement>,
    pub playable_races: Option<CharacterItemRaceRequirement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemLevelRequirement {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemClassRequirement {
    pub value: u32,
    pub display_string: String,
    pub playable_classes: Vec<CharacterItemClassInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemClassInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemRaceRequirement {
    pub value: u32,
    pub display_string: String,
    pub playable_races: Vec<CharacterItemRaceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemRaceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemLevel {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemTransmog {
    pub item: CharacterItemTransmogInfo,
    pub display_string: String,
    pub item_modified_appearance_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemTransmogInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemDurability {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSet {
    pub item_set: CharacterItemSetInfo,
    pub items: Vec<CharacterItemSetItem>,
    pub effects: Vec<CharacterItemSetEffect>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSetInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSetItem {
    pub item: CharacterItem,
    pub is_equipped: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSetEffect {
    pub display_string: String,
    pub required_count: u32,
    pub active_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSocket {
    pub socket_type: CharacterItemSocketType,
    pub item: CharacterItem,
    pub display_string: String,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSocketType {
    #[serde(rename = "type")]
    pub socket_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemEnchantment {
    pub enchantment_id: u32,
    pub enchantment_slot: CharacterItemEnchantmentSlot,
    pub display_string: String,
    pub source_item: Option<CharacterItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemEnchantmentSlot {
    pub id: u32,
    #[serde(rename = "type")]
    pub slot_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemWeapon {
    pub damage: CharacterItemWeaponDamage,
    pub attack_speed: CharacterItemWeaponAttackSpeed,
    pub dps: CharacterItemWeaponDps,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemWeaponDamage {
    pub min_value: u32,
    pub max_value: u32,
    pub display_string: String,
    pub damage_class: CharacterItemWeaponDamageClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemWeaponDamageClass {
    #[serde(rename = "type")]
    pub damage_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemWeaponAttackSpeed {
    pub value: f32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemWeaponDps {
    pub value: f32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItemSpellTooltip {
    pub spell: CharacterItemSpellInfo,
    pub description: String,
    pub cast_time: String,
    pub cooldown: Option<String>,
    pub range: Option<String>,
    pub power_cost: Option<String>,
}
