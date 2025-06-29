use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterAppearanceCharacter,
    pub playable_race: CharacterAppearanceRace,
    pub playable_class: CharacterAppearanceClass,
    pub active_spec: CharacterAppearanceSpecialization,
    pub gender: CharacterAppearanceGender,
    pub faction: CharacterAppearanceFaction,
    pub guild_crest: Option<CharacterAppearanceGuildCrest>,
    pub items: Vec<CharacterAppearanceItem>,
    pub customizations: Vec<CharacterAppearanceCustomization>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterAppearanceRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceRace {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceGender {
    #[serde(rename = "type")]
    pub gender_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceFaction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceGuildCrest {
    pub emblem: CharacterAppearanceGuildCrestEmblem,
    pub border: CharacterAppearanceGuildCrestBorder,
    pub background: CharacterAppearanceGuildCrestBackground,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceGuildCrestEmblem {
    pub id: u32,
    pub media: Href,
    pub color: CharacterAppearanceColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceGuildCrestBorder {
    pub id: u32,
    pub media: Href,
    pub color: CharacterAppearanceColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceGuildCrestBackground {
    pub color: CharacterAppearanceColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceColor {
    pub id: u32,
    pub rgba: CharacterAppearanceRgba,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItem {
    pub item: CharacterAppearanceItemInfo,
    pub slot: CharacterAppearanceItemSlot,
    pub enchant: Option<u32>,
    pub item_appearance: Option<CharacterAppearanceItemAppearance>,
    pub internal_slot_id: u32,
    pub subclass: u32,
    pub item_subclass: CharacterAppearanceItemSubclass,
    pub inventory_type: CharacterAppearanceInventoryType,
    pub binding: CharacterAppearanceItemBinding,
    pub armor: Option<CharacterAppearanceItemArmor>,
    pub weapon: Option<CharacterAppearanceItemWeapon>,
    pub stats: Option<Vec<CharacterAppearanceItemStat>>,
    pub spells: Option<Vec<CharacterAppearanceItemSpell>>,
    pub sell_price: Option<CharacterAppearanceItemSellPrice>,
    pub requirements: Option<CharacterAppearanceItemRequirements>,
    pub level: CharacterAppearanceItemLevel,
    pub transmog: Option<CharacterAppearanceItemTransmog>,
    pub durability: Option<CharacterAppearanceItemDurability>,
    pub unique_equipped: Option<String>,
    pub set: Option<CharacterAppearanceItemSet>,
    pub socket: Option<CharacterAppearanceItemSocket>,
    pub enchantments: Option<Vec<CharacterAppearanceItemEnchantment>>,
    pub bonus_list: Option<Vec<u32>>,
    pub name_description: Option<String>,
    pub name_description_color: Option<CharacterAppearanceColor>,
    pub media: Href,
    pub item_class: CharacterAppearanceItemClass,
    pub preview_item: Option<Href>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSlot {
    #[serde(rename = "type")]
    pub slot_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemAppearance {
    pub item: CharacterAppearanceItemInfo,
    pub item_appearance_modifier_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSubclass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceInventoryType {
    #[serde(rename = "type")]
    pub inventory_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemBinding {
    #[serde(rename = "type")]
    pub binding_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemArmor {
    pub value: u32,
    pub display: CharacterAppearanceItemArmorDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemArmorDisplay {
    pub display_string: String,
    pub color: CharacterAppearanceColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemWeapon {
    pub damage: CharacterAppearanceItemWeaponDamage,
    pub attack_speed: CharacterAppearanceItemWeaponAttackSpeed,
    pub dps: CharacterAppearanceItemWeaponDps,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemWeaponDamage {
    pub min_value: u32,
    pub max_value: u32,
    pub display_string: String,
    pub damage_class: CharacterAppearanceItemWeaponDamageClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemWeaponDamageClass {
    #[serde(rename = "type")]
    pub damage_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemWeaponAttackSpeed {
    pub value: f32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemWeaponDps {
    pub value: f32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemStat {
    #[serde(rename = "type")]
    pub stat_type: CharacterAppearanceItemStatType,
    pub value: i32,
    pub display: CharacterAppearanceItemStatDisplay,
    pub is_negated: Option<bool>,
    pub is_equip_bonus: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemStatType {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemStatDisplay {
    pub display_string: String,
    pub color: CharacterAppearanceColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSpell {
    pub spell: CharacterAppearanceItemSpellInfo,
    pub description: String,
    pub display_color: Option<CharacterAppearanceColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSpellInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSellPrice {
    pub value: u32,
    pub display_strings: CharacterAppearanceItemSellPriceDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSellPriceDisplay {
    pub header: String,
    pub gold: String,
    pub silver: String,
    pub copper: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemRequirements {
    pub level: Option<CharacterAppearanceItemLevelRequirement>,
    pub playable_classes: Option<CharacterAppearanceItemClassRequirement>,
    pub playable_races: Option<CharacterAppearanceItemRaceRequirement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemLevelRequirement {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemClassRequirement {
    pub value: u32,
    pub display_string: String,
    pub playable_classes: Vec<CharacterAppearanceItemClassInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemClassInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemRaceRequirement {
    pub value: u32,
    pub display_string: String,
    pub playable_races: Vec<CharacterAppearanceItemRaceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemRaceInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemLevel {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemTransmog {
    pub item: CharacterAppearanceItemTransmogInfo,
    pub display_string: String,
    pub item_modified_appearance_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemTransmogInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemDurability {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSet {
    pub item_set: CharacterAppearanceItemSetInfo,
    pub items: Vec<CharacterAppearanceItemSetItem>,
    pub effects: Vec<CharacterAppearanceItemSetEffect>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSetInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSetItem {
    pub item: CharacterAppearanceItemInfo,
    pub is_equipped: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSetEffect {
    pub display_string: String,
    pub required_count: u32,
    pub active_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSocket {
    pub socket_type: CharacterAppearanceItemSocketType,
    pub item: CharacterAppearanceItemInfo,
    pub display_string: String,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemSocketType {
    #[serde(rename = "type")]
    pub socket_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemEnchantment {
    pub enchantment_id: u32,
    pub enchantment_slot: CharacterAppearanceItemEnchantmentSlot,
    pub display_string: String,
    pub source_item: Option<CharacterAppearanceItemInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemEnchantmentSlot {
    pub id: u32,
    #[serde(rename = "type")]
    pub slot_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceItemClass {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceCustomization {
    pub option: CharacterAppearanceCustomizationOption,
    pub choice: CharacterAppearanceCustomizationChoice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceCustomizationOption {
    pub name: String,
    pub option_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAppearanceCustomizationChoice {
    pub name: String,
    pub choice_id: u32,
}
