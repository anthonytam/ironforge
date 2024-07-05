use serde::{Deserialize, Serialize};

use super::common::{Href, Links, TypeNode};

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub covenants: Vec<CovenantSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSummary {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Covenant {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub renown_rewards: Vec<CovenantRenownReward>,
    pub signature_ability: Option<CovenantSignatueAbility>,
    pub class_ability: Option<Vec<CovenantClassAbility>>,
    pub soulbinds: Option<Vec<SoulbindSummary>>,
    pub media: Option<CovenantMediaSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSignatueAbility {
    pub id: u32,
    pub spell_tooltip: CovenantSpellToolTip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSpellToolTip {
    pub spell: CovenantSpell,
    pub description: String,
    pub cast_time: String,
    pub power_cost: Option<String>,
    pub range: Option<String>,
    pub cooldown: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantClassAbility {
    pub id: u32,
    pub playable_class: CovenantClass,
    pub spell_tooltop: CovenantSpellToolTip
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSpell {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantClass {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantSignatureAbility {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindSummary{
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantRenownReward {
    pub level: u32,
    pub reward: CovenantRenownRewardLink,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantRenownRewardLink {
    pub key: Href
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantMediaSummary {
    pub key: Href,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantMedia {
    #[serde(rename = "_links")]
    pub links: Links,
    pub assets: Vec<CovenantMediaAsset>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub soulbinds: Vec<Soulbind>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Soulbind {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub covenant: Covenant,
    pub creature: SoulbindCreature,
    pub follower: SoulbindFollower,
    pub talent_tree: SoulbindTalentTree
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindCreature {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindFollower {
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulbindTalentTree {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub conduits: Vec<ConduitSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitSummary {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conduit {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub item: ConduitItem,
    pub socket_type: TypeNode,
    pub ranks: Vec<ConduitRank>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitItem {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitRank {
    pub id: u32,
    pub tier: u32,
    pub spell_tooltop: CovenantSpellToolTip
}