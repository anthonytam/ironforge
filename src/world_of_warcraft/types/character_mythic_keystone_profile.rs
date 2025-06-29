use serde::{Deserialize, Serialize};

use super::common::{Href, Links};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileIndexResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterMythicKeystoneProfileCharacter,
    pub current_period: CharacterMythicKeystoneProfilePeriod,
    pub seasons: Vec<CharacterMythicKeystoneProfileSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneSeasonDetailsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterMythicKeystoneProfileCharacter,
    pub season: CharacterMythicKeystoneProfileSeason,
    pub best_runs: Vec<CharacterMythicKeystoneProfileRun>,
    pub character_best_runs: Vec<CharacterMythicKeystoneProfileRun>,
    pub weekly_best_runs: Vec<CharacterMythicKeystoneProfileRun>,
    pub weekly_runs: Vec<CharacterMythicKeystoneProfileRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileCharacter {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub realm: CharacterMythicKeystoneProfileRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfilePeriod {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileSeason {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileRun {
    pub completed_timestamp: u64,
    pub duration: u64,
    pub keystone_level: u32,
    pub keystone_affixes: Vec<CharacterMythicKeystoneProfileAffix>,
    pub members: Vec<CharacterMythicKeystoneProfileMember>,
    pub dungeon: CharacterMythicKeystoneProfileDungeon,
    pub is_completed_within_time: bool,
    pub mythic_rating: CharacterMythicKeystoneProfileRating,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileAffix {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileMember {
    pub character: CharacterMythicKeystoneProfileMemberCharacter,
    pub specialization: CharacterMythicKeystoneProfileMemberSpecialization,
    pub race: CharacterMythicKeystoneProfileMemberRace,
    pub equipped_item_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileMemberCharacter {
    pub name: String,
    pub id: u32,
    pub realm: CharacterMythicKeystoneProfileMemberRealm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileMemberRealm {
    pub key: Href,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileMemberSpecialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileMemberRace {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileDungeon {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileRating {
    pub color: CharacterMythicKeystoneProfileRatingColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneProfileRatingColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}
