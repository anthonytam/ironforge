use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub covenants: Vec<CovenantSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub signature_ability: CovenantSignatureAbility,
    pub class_abilities: Vec<CovenantClassAbility>,
    pub soulbinds: Vec<CovenantSoulbind>,
    pub renown_rewards: Vec<CovenantRenownReward>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSignatureAbility {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantClassAbility {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSoulbind {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantRenownReward {
    pub level: u32,
    pub reward: CovenantReward,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantReward {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantMediaResponse {
    pub assets: Vec<CovenantMediaAsset>,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantMediaAsset {
    #[serde(rename = "file_data_id")]
    pub file_data_id: u32,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub conduits: Vec<ConduitSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub item: ConduitItem,
    pub socket_type: ConduitSocketType,
    pub ranks: Vec<ConduitRank>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitSocketType {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitRank {
    pub rank: u32,
    pub spell_tooltip: ConduitSpellTooltip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitSpellTooltip {
    pub spell: ConduitSpell,
    pub description: String,
    pub cast_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitSpell {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub soulbinds: Vec<SoulbindSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindSummary {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub covenant: SoulbindCovenant,
    pub creature: SoulbindCreature,
    pub follower: SoulbindFollower,
    pub talents: Vec<SoulbindTalent>,
    pub tree: SoulbindTree,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindCovenant {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindCreature {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindFollower {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTalent {
    pub talent: SoulbindTalentInfo,
    pub rank: u32,
    pub tooltip: SoulbindTalentTooltip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTalentInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTalentTooltip {
    pub talent: SoulbindTalentInfo,
    pub spell_tooltip: SoulbindSpellTooltip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindSpellTooltip {
    pub spell: SoulbindSpell,
    pub description: String,
    pub cast_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindSpell {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTree {
    pub id: u32,
    pub talents: Vec<SoulbindTreeTalent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTreeTalent {
    pub node: SoulbindTalentNode,
    pub rank: u32,
    pub tooltip: SoulbindTalentTooltip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTalentNode {
    pub id: u32,
    pub name: String,
    pub coordinates: SoulbindCoordinates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindCoordinates {
    pub x: f32,
    pub y: f32,
}
