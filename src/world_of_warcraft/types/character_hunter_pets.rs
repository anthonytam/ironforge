use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetsSummaryResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterHunterPetsCharacter,
    pub hunter_pets: Vec<CharacterHunterPet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetsCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterHunterPetsRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetsRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPet {
    pub name: String,
    pub level: u32,
    pub creature: CharacterHunterPetCreature,
    pub slot: u32,
    pub quality: CharacterHunterPetQuality,
    pub stats: CharacterHunterPetStats,
    pub specifications: Vec<CharacterHunterPetSpecification>,
    pub media: Href,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetCreature {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetQuality {
    #[serde(rename = "type")]
    pub quality_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetStats {
    pub species_id: u32,
    pub breed_id: u32,
    pub pet_quality_id: u32,
    pub level: u32,
    pub health: u32,
    pub power: u32,
    pub speed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetSpecification {
    pub specialization: CharacterHunterPetSpecificationInfo,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterHunterPetSpecificationInfo {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
