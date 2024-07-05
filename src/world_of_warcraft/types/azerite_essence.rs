use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssenceIndex {
    #[serde(rename = "_links")]
    pub links: Links,
    pub azerite_essences: Vec<AzeriteEssence>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssence {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssenceDetail {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: u32,
    pub name: String,
    pub allowed_specializations: Vec<AzeriteEssenceSpecialization>,
    pub powers: Vec<AzeriteEssencePower>,
    pub media: AzeriteEssenceMedia 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssenceSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssencePower {
    pub id: u32,
    pub rank: u32,
    pub main_power_spell: AzeriteEssencePowerSpell,
    pub passive_power_spell: AzeriteEssencePowerSpell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssencePowerSpell {
    pub key: Href,
    pub name: String,
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssenceMedia {
    pub key: Href,
    pub id: u32
}