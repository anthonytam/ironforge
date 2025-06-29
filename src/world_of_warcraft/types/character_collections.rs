use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionsIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub collections: CharacterCollectionsLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMountsCollectionSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub mounts: Vec<CharacterMount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPetsCollectionSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub pets: Vec<CharacterPet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterToysCollectionSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub toys: Vec<CharacterToy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHeirloomsCollectionSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub heirlooms: Vec<CharacterHeirloom>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTransmogCollectionSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterCollectionsCharacter,
    pub collected_appearances: Vec<CharacterTransmogAppearance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterCollectionsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionsLinks {
    pub mounts: Href,
    pub pets: Href,
    pub toys: Href,
    pub heirlooms: Href,
    pub transmog: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMount {
    pub mount: CharacterMountInfo,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMountInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPet {
    pub species: CharacterPetSpecies,
    pub level: u32,
    pub quality: CharacterPetQuality,
    pub stats: CharacterPetStats,
    pub creature_display: Option<CharacterPetCreatureDisplay>,
    pub id: u32,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPetSpecies {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPetQuality {
    #[serde(rename = "type")]
    pub quality_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPetStats {
    pub species_id: u32,
    pub breed_id: u32,
    pub pet_quality_id: u32,
    pub level: u32,
    pub health: u32,
    pub power: u32,
    pub speed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPetCreatureDisplay {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterToy {
    pub item: CharacterToyItem,
    pub has_favorite_appearance: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterToyItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHeirloom {
    pub item: CharacterHeirloomItem,
    pub upgrade_bonuses: Vec<CharacterHeirloomUpgradeBonus>,
    pub upgrade_bonus_levels: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHeirloomItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHeirloomUpgradeBonus {
    pub upgrade_level: u32,
    pub item_bonus: CharacterHeirloomItemBonus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHeirloomItemBonus {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTransmogAppearance {
    pub item: CharacterTransmogItem,
    pub item_appearance_modifier_id: u32,
    pub transmog_item_appearance_modifier_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterTransmogItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
