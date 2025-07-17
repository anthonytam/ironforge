use crate::api_client::{ApiRequestHelper, BlizzardAPIClient, BlizzardAPIClientError};
use serde::Deserialize;
use time::{OffsetDateTime, format_description::well_known::Rfc2822};
use tracing::{debug, error, info, instrument};
use types::common::Href;

pub mod types;

pub mod achievements;
pub mod auction_house;
pub mod azerite_essence;
pub mod character_achievements;
pub mod character_appearance;
pub mod character_collections;
pub mod character_encounters;
pub mod character_equipment;
pub mod character_hunter_pets;
pub mod character_media;
pub mod character_mythic_keystone_profile;
pub mod character_professions;
pub mod character_profile;
pub mod character_pvp;
pub mod character_quests;
pub mod character_reputations;
pub mod character_soulbinds;
pub mod character_specializations;
pub mod character_statistics;
pub mod character_titles;
pub mod connected_realm;
pub mod covenant;
pub mod creature;
pub mod guild;
pub mod guild_crest;
pub mod heirloom;
pub mod item;
pub mod journal;
pub mod media_search;
pub mod modified_crafting;
pub mod mount;
pub mod mythic_keystone_affix;
pub mod mythic_keystone_dungeon;
pub mod mythic_keystone_leaderboard;
pub mod mythic_raid_leaderboard;
pub mod pet;
pub mod playable_class;
pub mod playable_race;
pub mod playable_specialization;
pub mod power_type;
pub mod profession;
pub mod pvp_season;
pub mod pvp_tier;
pub mod quest;
pub mod realm;
pub mod region;
pub mod reputations;
pub mod spell;
pub mod talent;
pub mod tech_talent;
pub mod title;
pub mod toy;
pub mod wow_token;

#[derive(Clone)]
pub struct WorldOfWarcraftClient {
    pub client: BlizzardAPIClient,
}

impl WorldOfWarcraftClient {
    pub fn get(client: BlizzardAPIClient) -> WorldOfWarcraftClient {
        info!("Creating World of Warcraft client");
        WorldOfWarcraftClient { client }
    }

    #[instrument(skip(self), fields(href = %href.href), level = "info")]
    pub async fn get_last_update(
        &self,
        href: Href,
    ) -> Result<OffsetDateTime, BlizzardAPIClientError> {
        info!("Getting last update time for HREF: {}", href.href);

        let response = self.client.send_request_to_href(href.clone()).await?;

        debug!("Extracting Last-Modified header from response");
        let last_modified_string = response
            .headers()
            .get("Last-Modified")
            .ok_or_else(|| {
                BlizzardAPIClientError::MissingHeaderError(
                    "Last-Modified header not found in response".to_string(),
                )
            })?
            .to_str()
            .map_err(|e| {
                BlizzardAPIClientError::DeserializationError(format!(
                    "Failed to decode Last-Modified header: {e}"
                ))
            })?;

        debug!("Parsing Last-Modified date: {}", last_modified_string);

        let result = OffsetDateTime::parse(last_modified_string, &Rfc2822).map_err(|e| {
            BlizzardAPIClientError::DateParsingError(format!(
                "Failed to parse Last-Modified date '{last_modified_string}': {e}"
            ))
        });

        match &result {
            Ok(datetime) => info!("Successfully parsed last update time: {}", datetime),
            Err(e) => error!("Failed to parse last update time: {:?}", e),
        }

        result
    }

    #[instrument(skip(self), fields(href = %href.href), level = "info")]
    pub async fn get_href_data<T: for<'de> Deserialize<'de>>(
        &self,
        href: &Href,
    ) -> Result<T, BlizzardAPIClientError> {
        info!("Getting HREF data for: {}", href.href);
        let result = self.client.request_href_and_deserialize(href.clone()).await;

        match &result {
            Ok(_) => info!("Successfully retrieved HREF data for: {}", href.href),
            Err(e) => error!("Failed to retrieve HREF data for {}: {:?}", href.href, e),
        }

        result
    }
}

#[cfg(test)]
mod test_utils;