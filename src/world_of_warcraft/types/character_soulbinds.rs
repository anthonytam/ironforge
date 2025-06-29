use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterSoulbindsCharacter,
    pub soulbinds: Vec<CharacterSoulbind>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterSoulbindsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbind {
    pub soulbind: CharacterSoulbindInfo,
    pub traits: Vec<CharacterSoulbindTrait>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindTrait {
    #[serde(rename = "trait")]
    pub trait_info: CharacterSoulbindTraitInfo,
    pub tier: u32,
    pub display_order: u32,
    pub conduit_socket: Option<CharacterSoulbindConduitSocket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindTraitInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindConduitSocket {
    pub socket: CharacterSoulbindConduitSocketInfo,
    pub rank: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSoulbindConduitSocketInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
