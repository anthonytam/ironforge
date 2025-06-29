use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpTalentIndexResponse {
    #[serde(rename = "pvp_talents")]
    pub pvp_talents: Vec<TalentSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PvpTalentResponse {
    #[serde(rename = "compatible_slots")]
    pub compatible_slots: Vec<u32>,
    pub description: String,
    pub id: u32,
    #[serde(rename = "playable_specialization")]
    pub playable_specialization: SpecializationSummary,
    pub spell: SpellSummary,
    #[serde(rename = "unlock_player_level")]
    pub unlock_player_level: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentIndexResponse {
    pub talents: Vec<TalentSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentResponse {
    pub id: u32,
    #[serde(rename = "playable_class")]
    pub playable_class: ClassSummary,
    #[serde(rename = "rank_descriptions")]
    pub rank_descriptions: Vec<RankDescription>,
    pub spell: SpellSummary,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentTreeIndexResponse {
    #[serde(rename = "class_talent_trees")]
    pub class_talent_trees: Vec<TalentTree>,
    #[serde(rename = "spec_talent_trees")]
    pub spec_talent_trees: Vec<TalentTree>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentTreeNodesResponse {
    pub id: u32,
    #[serde(rename = "spec_talent_trees")]
    pub spec_talent_trees: Vec<SpecTalentTree>,
    #[serde(rename = "talent_nodes")]
    pub talent_nodes: Vec<TalentNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentTreeResponse {
    pub key: Key,
    pub name: String,
    pub id: u32,
    #[serde(rename = "class_talent_nodes")]
    pub class_talent_nodes: Vec<ClassTalentNode>,
    pub media: MediaLink,
    #[serde(rename = "playable_class")]
    pub playable_class: ClassSummary,
    #[serde(rename = "playable_specialization")]
    pub playable_specialization: SpecializationSummary,
    #[serde(rename = "restriction_lines")]
    pub restriction_lines: Vec<RestrictionLine>,
    #[serde(rename = "spec_talent_nodes")]
    pub spec_talent_nodes: Vec<SpecTalentNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassTalentNode {
    #[serde(rename = "display_col")]
    pub display_col: u32,
    #[serde(rename = "display_row")]
    pub display_row: u32,
    pub id: u32,
    #[serde(rename = "locked_by")]
    pub locked_by: Option<Vec<u32>>,
    #[serde(rename = "node_type")]
    pub node_type: NodeType,
    pub ranks: Vec<ClassTalentNodeRank>,
    #[serde(rename = "raw_position_x")]
    pub raw_position_x: f64,
    #[serde(rename = "raw_position_y")]
    pub raw_position_y: f64,
    pub unlocks: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassTalentNodeRank {
    #[serde(rename = "choice_of_tooltips")]
    pub choice_of_tooltips: Option<Vec<Tooltip>>,
    #[serde(rename = "default_points")]
    pub default_points: Option<u32>,
    pub rank: u32,
    pub tooltip: Option<Tooltip>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeType {
    pub id: u32,
    #[serde(rename = "type")]
    pub node_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RankDescription {
    pub description: Option<String>,
    pub rank: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RestrictionLine {
    #[serde(rename = "is_for_class")]
    pub is_for_class: bool,
    #[serde(rename = "required_points")]
    pub required_points: u32,
    #[serde(rename = "restricted_row")]
    pub restricted_row: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecTalentNode {
    #[serde(rename = "display_col")]
    pub display_col: u32,
    #[serde(rename = "display_row")]
    pub display_row: u32,
    pub id: u32,
    #[serde(rename = "locked_by")]
    pub locked_by: Option<Vec<u32>>,
    #[serde(rename = "node_type")]
    pub node_type: NodeType,
    pub ranks: Vec<SpecTalentNodeRank>,
    #[serde(rename = "raw_position_x")]
    pub raw_position_x: f64,
    #[serde(rename = "raw_position_y")]
    pub raw_position_y: f64,
    pub unlocks: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecTalentNodeRank {
    pub rank: u32,
    pub tooltip: Option<Tooltip>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecTalentTree {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentNode {
    #[serde(rename = "display_col")]
    pub display_col: u32,
    #[serde(rename = "display_row")]
    pub display_row: u32,
    pub id: u32,
    #[serde(rename = "node_type")]
    pub node_type: NodeType,
    pub ranks: Vec<Rank>,
    #[serde(rename = "raw_position_x")]
    pub raw_position_x: f64,
    #[serde(rename = "raw_position_y")]
    pub raw_position_y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rank {
    pub rank: u32,
    pub tooltip: Option<Tooltip>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tooltip {
    #[serde(rename = "spell_tooltip")]
    pub spell_tooltip: SpellTooltip,
    pub talent: TalentSummary,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpellTooltip {
    #[serde(rename = "cast_time")]
    pub cast_time: String,
    pub cooldown: Option<String>,
    pub description: String,
    #[serde(rename = "power_cost")]
    pub power_cost: Option<String>,
    pub range: Option<String>,
    pub spell: SpellSummary,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaLink {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecializationSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpellSummary {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TalentTree {
    pub key: Key,
    pub name: String,
    pub id: u32,
}
