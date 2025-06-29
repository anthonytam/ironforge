use serde::{Deserialize, Serialize};

use super::common::{Color, Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub heirlooms: Vec<HeirloomSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heirloom {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub item: HeirloomItem,
    pub source: HeirloomSource,
    pub source_description: String,
    pub upgrades: Vec<HeirloomUpgrade>,
    pub media: HeirloomMedia,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomItem {
    pub key: Href,
    pub name: Option<String>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgrade {
    pub item: HeirloomUpgradeItem,
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeItem {
    pub item: HeirloomItem,
    pub context: u32,
    pub bonus_list: Vec<u32>,
    pub quality: TypeNode,
    pub name: String,
    pub media: HeirloomUpgradeMedia,
    pub item_class: HeirloomUpgradeItemClass,
    pub item_subclass: HeirloomUpgradeItemClass,
    pub inventory_type: TypeNode,
    pub binding_type: TypeNode,
    pub weapon: Option<HeirloomWeapon>,
    pub stats: Vec<HeirloomStat>,
    pub upgrades: HeirloomUpgradeUpgrade,
    pub requirements: HeirloomUpgradeRequirements,
    pub level: HeirloomUpgradeLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeItemClass {
    pub key: Href,
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomWeapon {
    pub damage: HeirloomWeaponDamage,
    pub attack_speed: HeirloomWeaponAttackSpeed,
    pub dps: HeirloomWeaponDPS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomWeaponDamage {
    pub min_value: u32,
    pub max_value: u32,
    pub display_string: String,
    pub damage_class: TypeNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomWeaponAttackSpeed {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomWeaponDPS {
    pub value: f32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomStat {
    #[serde(rename = "type")]
    pub stat_type: TypeNode,
    pub value: u32,
    pub display: HeirloomStatDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomStatDisplay {
    #[serde(rename = "type")]
    pub display_string: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeUpgrade {
    pub value: u32,
    pub max_value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeRequirements {
    pub level: HeirloomUpgradeRequirementsLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeRequirementsLevel {
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgradeLevel {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomMedia {
    pub key: Href,
    pub id: u32,
}
