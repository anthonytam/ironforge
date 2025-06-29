use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use reqwest::Response;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use tracing::{Level, debug, error, info, instrument, warn};
use url::Url;

use crate::{types::BlizzardAccessTokenResponse, world_of_warcraft::types::common::Href};

type TokenRefreshReceiver = tokio::sync::oneshot::Receiver<Result<(), BlizzardAPIClientError>>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Region {
    US,
    EU,
    KR,
    TW,
    CN,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Locale {
    en_US,
    es_MX,
    pt_BR,
    en_GB,
    es_ES,
    fr_FR,
    ru_RU,
    de_DE,
    pt_PT,
    it_IT,
    ko_KR,
    zh_TW,
    zh_CN,
}

#[derive(Clone, Debug)]
pub struct BlizzardAPIClient {
    reqwest_client: reqwest::Client,
    region: Region,
    locale: Locale,
    client_id: String,
    client_secret: String,
    access_token: Arc<RwLock<Option<String>>>,
    token_expiration: Arc<RwLock<SystemTime>>,
    refresh_in_progress:
        Arc<Mutex<Option<TokenRefreshReceiver>>>,
}

pub struct BlizzardAPIClientBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    region: Option<Region>,
    locale: Option<Locale>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    connection_pool_size: Option<usize>,
    enable_tracing: Option<bool>,
    tracing_level: Option<Level>,
}

impl Default for BlizzardAPIClientBuilder {
    fn default() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            region: Some(Region::US),
            locale: Some(Locale::en_US),
            timeout: Some(Duration::from_secs(30)),
            user_agent: Some("ironforge/0.2.0".to_string()),
            connection_pool_size: Some(10),
            enable_tracing: None,
            tracing_level: None,
        }
    }
}

impl BlizzardAPIClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn connection_pool_size(mut self, pool_size: usize) -> Self {
        self.connection_pool_size = Some(pool_size);
        self
    }

    pub fn enable_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = Some(enable);
        self
    }

    pub fn tracing_level(mut self, level: Level) -> Self {
        self.tracing_level = Some(level);
        self
    }

    pub fn build(self) -> Result<BlizzardAPIClient, BlizzardAPIClientError> {
        let client_id = self.client_id.ok_or_else(|| {
            BlizzardAPIClientError::ConfigurationError("Client ID is required".to_string())
        })?;

        let client_secret = self.client_secret.ok_or_else(|| {
            BlizzardAPIClientError::ConfigurationError("Client secret is required".to_string())
        })?;

        let region = self.region.ok_or_else(|| {
            BlizzardAPIClientError::ConfigurationError("Region is required".to_string())
        })?;

        let locale = self.locale.ok_or_else(|| {
            BlizzardAPIClientError::ConfigurationError("Locale is required".to_string())
        })?;

        let mut client_builder = reqwest::ClientBuilder::new();

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        if let Some(user_agent) = self.user_agent {
            client_builder = client_builder.user_agent(user_agent);
        }

        if let Some(pool_size) = self.connection_pool_size {
            client_builder = client_builder.pool_max_idle_per_host(pool_size);
        }

        if self.enable_tracing.unwrap_or(false) {
            info!("Tracing enabled for Blizzard API client");
        }

        let reqwest_client = client_builder.build().map_err(|e| {
            BlizzardAPIClientError::ClientInitializationError(format!(
                "Failed to build HTTP client: {e}"
            ))
        })?;

        info!(
            "Blizzard API client initialized for region: {:?}, locale: {:?}",
            region, locale
        );

        Ok(BlizzardAPIClient {
            reqwest_client,
            region,
            locale,
            client_id,
            client_secret,
            access_token: Arc::new(RwLock::new(None)),
            token_expiration: Arc::new(RwLock::new(SystemTime::now())),
            refresh_in_progress: Arc::new(Mutex::new(None)),
        })
    }
}

#[derive(Error, Debug, Clone)]
pub enum BlizzardAPIClientError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Client initialization failed: {0}")]
    ClientInitializationError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Access token fetch failed: {0}")]
    AccessTokenFetchError(String),
    #[error("Access token deserialization failed: {0}")]
    AccessTokenDeserializationError(String),
    #[error("Access token expired or invalid: {0}")]
    AccessTokenExpiredError(String),
    #[error("HTTP request failed: {status} - {message}")]
    HttpRequestError {
        status: u16,
        message: String,
        endpoint: String,
    },

    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Request timeout: {0}")]
    RequestTimeoutError(String),
    #[error("Rate limited: {0}")]
    RateLimitError(String),
    #[error("Response deserialization failed: {0}")]
    DeserializationError(String),
    #[error("Missing required header: {0}")]
    MissingHeaderError(String),
    #[error("Invalid response format: {0}")]
    InvalidResponseError(String),
    #[error("Date parsing failed: {0}")]
    DateParsingError(String),
    #[error("Timezone conversion failed: {0}")]
    TimezoneError(String),
    #[error("API endpoint not found: {0}")]
    EndpointNotFoundError(String),
    #[error("API service unavailable: {0}")]
    ServiceUnavailableError(String),
    #[error("API internal error: {0}")]
    ApiInternalError(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl BlizzardAPIClientError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            BlizzardAPIClientError::NetworkError(_)
                | BlizzardAPIClientError::RequestTimeoutError(_)
                | BlizzardAPIClientError::RateLimitError(_)
                | BlizzardAPIClientError::ServiceUnavailableError(_)
                | BlizzardAPIClientError::ApiInternalError(_)
        )
    }

    pub fn user_message(&self) -> String {
        match self {
            BlizzardAPIClientError::ConfigurationError(msg) => {
                format!("Configuration issue: {msg}")
            }
            BlizzardAPIClientError::AuthenticationError(msg) => {
                format!("Authentication failed: {msg}")
            }
            BlizzardAPIClientError::AccessTokenExpiredError(_) => {
                "Access token has expired. Please check your credentials.".to_string()
            }
            BlizzardAPIClientError::HttpRequestError {
                status,
                message,
                endpoint,
            } => format!("HTTP {status} error for {endpoint}: {message}"),
            BlizzardAPIClientError::NetworkError(msg) => {
                format!("Network connection issue: {msg}")
            }
            BlizzardAPIClientError::RequestTimeoutError(_) => {
                "Request timed out. Please try again.".to_string()
            }
            BlizzardAPIClientError::RateLimitError(_) => {
                "Rate limit exceeded. Please wait before making more requests.".to_string()
            }
            BlizzardAPIClientError::DeserializationError(msg) => {
                format!("Failed to parse API response: {msg}")
            }
            BlizzardAPIClientError::EndpointNotFoundError(endpoint) => {
                format!("API endpoint not found: {endpoint}")
            }
            BlizzardAPIClientError::ServiceUnavailableError(_) => {
                "Blizzard API service is temporarily unavailable.".to_string()
            }
            BlizzardAPIClientError::ApiInternalError(_) => {
                "Blizzard API encountered an internal error.".to_string()
            }
            _ => format!("An unexpected error occurred: {self}"),
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait ApiRequestHelper {
    async fn request_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        api_path: String,
        namespace: &str,
    ) -> Result<T, BlizzardAPIClientError>;

    async fn request_href_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        href: Href,
    ) -> Result<T, BlizzardAPIClientError>;
}

impl ApiRequestHelper for BlizzardAPIClient {
    #[instrument(skip(self), fields(endpoint = %api_path, namespace = %namespace), level = "info")]
    async fn request_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        api_path: String,
        namespace: &str,
    ) -> Result<T, BlizzardAPIClientError> {
        info!(
            "Requesting and deserializing data from endpoint: {}",
            api_path
        );

        let response = self.send_request(api_path.clone(), namespace).await?;

        debug!("Deserializing response from endpoint: {}", api_path);
        let result = response.json::<T>().await.map_err(|e| {
            BlizzardAPIClientError::DeserializationError(format!(
                "Failed to deserialize response from {api_path}: {e}"
            ))
        });

        match &result {
            Ok(_) => info!(
                "Successfully deserialized response from endpoint: {}",
                api_path
            ),
            Err(e) => error!(
                "Failed to deserialize response from endpoint {}: {:?}",
                api_path, e
            ),
        }

        result
    }

    #[instrument(skip(self), fields(href = %href.href), level = "info")]
    async fn request_href_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        href: Href,
    ) -> Result<T, BlizzardAPIClientError> {
        info!("Requesting and deserializing data from HREF: {}", href.href);

        let response = self.send_request_to_href(href.clone()).await?;

        debug!("Deserializing response from HREF: {}", href.href);
        let result = response.json::<T>().await.map_err(|e| {
            BlizzardAPIClientError::DeserializationError(format!(
                "Failed to deserialize response from href {}: {}",
                href.href, e
            ))
        });

        match &result {
            Ok(_) => info!(
                "Successfully deserialized response from HREF: {}",
                href.href
            ),
            Err(e) => error!(
                "Failed to deserialize response from HREF {}: {:?}",
                href.href, e
            ),
        }

        result
    }
}

impl BlizzardAPIClient {
    pub fn builder() -> BlizzardAPIClientBuilder {
        BlizzardAPIClientBuilder::new()
    }

    pub fn new(
        client_id: String,
        client_secret: String,
        region: Region,
        locale: Locale,
    ) -> Result<BlizzardAPIClient, BlizzardAPIClientError> {
        BlizzardAPIClientBuilder::new()
            .client_id(client_id)
            .client_secret(client_secret)
            .region(region)
            .locale(locale)
            .build()
    }

    #[instrument(skip(self), level = "debug")]
    async fn get_new_access_token(&self) -> Result<bool, BlizzardAPIClientError> {
        let token_url = self.get_token_url();
        debug!("Fetching new access token from: {}", token_url);

        let new_token_response = self
            .reqwest_client
            .post(token_url.as_str())
            .form(&[("grant_type", "client_credentials")])
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await;

        match new_token_response {
            Ok(response) => {
                let status = response.status();
                debug!("Token endpoint response status: {}", status);

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    error!("Token fetch failed with status {}: {}", status, error_text);

                    return match status {
                        401 => Err(BlizzardAPIClientError::AuthenticationError(format!(
                            "Invalid client credentials for token endpoint: {error_text}"
                        ))),
                        429 => Err(BlizzardAPIClientError::RateLimitError(format!(
                            "Rate limited when fetching access token: {error_text}"
                        ))),
                        503 => Err(BlizzardAPIClientError::ServiceUnavailableError(format!(
                            "Blizzard OAuth service unavailable: {error_text}"
                        ))),
                        _ => Err(BlizzardAPIClientError::HttpRequestError {
                            status,
                            message: error_text,
                            endpoint: token_url.to_string(),
                        }),
                    };
                }

                match response.json::<BlizzardAccessTokenResponse>().await {
                    Ok(blizzard_access_token_response) => {
                        debug!(
                            "Successfully parsed access token response, expires in {} seconds",
                            blizzard_access_token_response.expires_in
                        );

                        *self.access_token.write().await =
                            Some(blizzard_access_token_response.access_token);
                        *self.token_expiration.write().await = SystemTime::now()
                            .checked_add(Duration::from_secs(
                                blizzard_access_token_response.expires_in,
                            ))
                            .ok_or_else(|| {
                                BlizzardAPIClientError::AccessTokenExpiredError(
                                    "Token expiration time overflow".to_string(),
                                )
                            })?;

                        info!("Access token refreshed successfully");
                        Ok(true)
                    }
                    Err(e) => {
                        error!("Failed to parse access token response: {}", e);
                        Err(BlizzardAPIClientError::AccessTokenDeserializationError(
                            format!("Failed to parse access token response: {e}"),
                        ))
                    }
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    warn!("Timeout while fetching access token from {}", token_url);
                    Err(BlizzardAPIClientError::RequestTimeoutError(format!(
                        "Timeout while fetching access token from {token_url}"
                    )))
                } else if e.is_connect() {
                    error!("Network error while connecting to token endpoint: {}", e);
                    Err(BlizzardAPIClientError::NetworkError(format!(
                        "Network error while connecting to token endpoint: {e}"
                    )))
                } else {
                    error!("Failed to fetch access token from {}: {}", token_url, e);
                    Err(BlizzardAPIClientError::AccessTokenFetchError(format!(
                        "Failed to fetch access token from {token_url}: {e}"
                    )))
                }
            }
        }
    }

    async fn is_access_token_valid(&self) -> bool {
        let token = self.access_token.read().await;
        if token.is_none() {
            return false;
        }
        let expiration = self.token_expiration.read().await;
        expiration.ge(&SystemTime::now()
            .checked_add(Duration::from_secs(30))
            .unwrap_or(SystemTime::now()))
    }

    #[instrument(skip(self), level = "debug")]
    async fn ensure_valid_token(&self) -> Result<(), BlizzardAPIClientError> {
        if self.is_access_token_valid().await {
            debug!("Access token is still valid, no refresh needed");
            return Ok(());
        }

        debug!("Access token is invalid or expired, attempting refresh");
        let mut refresh_guard = self.refresh_in_progress.lock().await;

        if let Some(refresh_receiver) = refresh_guard.take() {
            debug!("Token refresh already in progress, waiting for completion");
            drop(refresh_guard);
            refresh_receiver.await.map_err(|_| {
                error!("Token refresh was cancelled");
                BlizzardAPIClientError::AccessTokenFetchError(
                    "Token refresh was cancelled".to_string(),
                )
            })?
        } else {
            debug!("Initiating new token refresh");
            let (tx, rx) = tokio::sync::oneshot::channel();
            *refresh_guard = Some(rx);
            drop(refresh_guard);

            match self.get_new_access_token().await {
                Ok(_) => {
                    debug!("Token refresh completed successfully");
                    let _ = tx.send(Ok(()));
                    Ok(())
                }
                Err(e) => {
                    error!("Token refresh failed: {:?}", e);
                    let _ = tx.send(Err(e.clone()));
                    Err(e)
                }
            }
        }
    }

    #[instrument(skip(self), fields(endpoint = %api_path, namespace = %namespace), level = "info")]
    pub async fn send_request(
        &self,
        api_path: String,
        namespace: &str,
    ) -> Result<Response, BlizzardAPIClientError> {
        info!("Sending API request to endpoint: {}", api_path);

        self.ensure_valid_token().await?;

        let access_token = {
            let guard = self.access_token.read().await;
            guard.clone().ok_or_else(|| {
                BlizzardAPIClientError::AccessTokenExpiredError(
                    "No access token available".to_string(),
                )
            })?
        };

        let mut api_url = self.get_api_url();
        api_url.set_path(&api_path);
        api_url
            .query_pairs_mut()
            .append_pair("locale", &self.locale.to_string());

        debug!("Making HTTP request to: {}", api_url);

        let response = self
            .reqwest_client
            .get(api_url.as_str())
            .header(
                "Battlenet-Namespace",
                format!("{}-{}", namespace, self.region),
            )
            .bearer_auth(access_token)
            .send()
            .await;

        match response {
            Ok(response) => {
                let status = response.status();
                debug!("Received response with status: {}", status);

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    warn!("API request failed with status {}: {}", status, error_text);

                    return match status {
                        401 => Err(BlizzardAPIClientError::AuthenticationError(format!(
                            "Invalid or expired access token for endpoint {api_path}: {error_text}"
                        ))),
                        403 => Err(BlizzardAPIClientError::AuthenticationError(format!(
                            "Access forbidden for endpoint {api_path}: {error_text}"
                        ))),
                        404 => Err(BlizzardAPIClientError::EndpointNotFoundError(api_path)),
                        429 => Err(BlizzardAPIClientError::RateLimitError(format!(
                            "Rate limited for endpoint {api_path}: {error_text}"
                        ))),
                        503 => Err(BlizzardAPIClientError::ServiceUnavailableError(format!(
                            "Blizzard API service unavailable for endpoint {api_path}: {error_text}"
                        ))),
                        500..=599 => Err(BlizzardAPIClientError::ApiInternalError(format!(
                            "Blizzard API internal error for endpoint {api_path}: {error_text}"
                        ))),
                        _ => Err(BlizzardAPIClientError::HttpRequestError {
                            status,
                            message: error_text,
                            endpoint: api_path,
                        }),
                    };
                }

                info!("API request completed successfully with status: {}", status);
                Ok(response)
            }
            Err(e) => {
                if e.is_timeout() {
                    warn!("Request timeout for endpoint {}: {}", api_path, e);
                    Err(BlizzardAPIClientError::RequestTimeoutError(format!(
                        "Request timeout for endpoint {api_path}: {e}"
                    )))
                } else if e.is_connect() {
                    error!("Network error for endpoint {}: {}", api_path, e);
                    Err(BlizzardAPIClientError::NetworkError(format!(
                        "Network error for endpoint {api_path}: {e}"
                    )))
                } else {
                    error!("Request failed for endpoint {}: {}", api_path, e);
                    Err(BlizzardAPIClientError::NetworkError(format!(
                        "Request failed for endpoint {api_path}: {e}"
                    )))
                }
            }
        }
    }

    #[instrument(skip(self), fields(href = %href.href), level = "info")]
    pub async fn send_request_to_href(
        &self,
        href: Href,
    ) -> Result<Response, BlizzardAPIClientError> {
        info!("Sending HREF request to: {}", href.href);

        self.ensure_valid_token().await?;

        let access_token = {
            let guard = self.access_token.read().await;
            guard.clone().ok_or_else(|| {
                BlizzardAPIClientError::AccessTokenExpiredError(
                    "No access token available".to_string(),
                )
            })?
        };

        let mut request_url = Url::parse(&href.href).map_err(|e| {
            BlizzardAPIClientError::InvalidResponseError(format!("Invalid HREF URL: {e}"))
        })?;

        request_url
            .query_pairs_mut()
            .append_pair("locale", &self.locale.to_string());

        debug!("Making HTTP request to HREF: {}", request_url);

        let response = self
            .reqwest_client
            .get(request_url.as_str())
            .bearer_auth(access_token)
            .send()
            .await;

        match response {
            Ok(response) => {
                let status = response.status();
                debug!("Received HREF response with status: {}", status);

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    warn!("HREF request failed with status {}: {}", status, error_text);

                    return match status {
                        401 => Err(BlizzardAPIClientError::AuthenticationError(format!(
                            "Invalid or expired access token for href {}: {}",
                            href.href, error_text
                        ))),
                        403 => Err(BlizzardAPIClientError::AuthenticationError(format!(
                            "Access forbidden for href {}: {}",
                            href.href, error_text
                        ))),
                        404 => Err(BlizzardAPIClientError::EndpointNotFoundError(href.href)),
                        429 => Err(BlizzardAPIClientError::RateLimitError(format!(
                            "Rate limited for href {}: {}",
                            href.href, error_text
                        ))),
                        503 => Err(BlizzardAPIClientError::ServiceUnavailableError(format!(
                            "Blizzard API service unavailable for href {}: {}",
                            href.href, error_text
                        ))),
                        500..=599 => Err(BlizzardAPIClientError::ApiInternalError(format!(
                            "Blizzard API internal error for href {}: {}",
                            href.href, error_text
                        ))),
                        _ => Err(BlizzardAPIClientError::HttpRequestError {
                            status,
                            message: error_text,
                            endpoint: href.href,
                        }),
                    };
                }

                info!(
                    "HREF request completed successfully with status: {}",
                    status
                );
                Ok(response)
            }
            Err(e) => {
                if e.is_timeout() {
                    warn!("Request timeout for href {}: {}", href.href, e);
                    Err(BlizzardAPIClientError::RequestTimeoutError(format!(
                        "Request timeout for href {}: {}",
                        href.href, e
                    )))
                } else if e.is_connect() {
                    error!("Network error for href {}: {}", href.href, e);
                    Err(BlizzardAPIClientError::NetworkError(format!(
                        "Network error for href {}: {}",
                        href.href, e
                    )))
                } else {
                    error!("Request failed for href {}: {}", href.href, e);
                    Err(BlizzardAPIClientError::NetworkError(format!(
                        "Request failed for href {}: {}",
                        href.href, e
                    )))
                }
            }
        }
    }

    fn get_api_url(&self) -> Url {
        let base_url = match self.region {
            Region::US | Region::EU | Region::KR | Region::TW => {
                format!("https://{}.api.blizzard.com", self.region)
            }
            Region::CN => "https://gateway.battlenet.com.cn".to_string(),
        };

        Url::parse(&base_url).expect("Invalid API base URL")
    }

    fn get_token_url(&self) -> Url {
        let token_url = match self.region {
            Region::US | Region::EU | Region::KR | Region::TW => {
                "https://oauth.battle.net/token".to_string()
            }
            Region::CN => "https://oauth.battlenet.com.cn/token".to_string(),
        };

        Url::parse(&token_url).expect("Invalid token URL")
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use tokio;

    use crate::world_of_warcraft::WorldOfWarcraftClient;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        dotenv().ok();

        let blizzard_client = BlizzardAPIClient::builder()
            .client_id(env::var("BLIZZARD_CLIENT_ID").expect("BLIZZARD_CLIENT_ID is not set."))
            .client_secret(
                env::var("BLIZZARD_CLIENT_SECRET").expect("BLIZZARD_CLIENT_SECRET is not set."),
            )
            .region(Region::US)
            .locale(Locale::en_US)
            .timeout(Duration::from_secs(60))
            .user_agent("ironforge-test/0.2.0")
            .connection_pool_size(20)
            .build()
            .expect("Failed to create BlizzardAPIClient");

        let wow_client = WorldOfWarcraftClient::get(blizzard_client);
        let _achievements_index = wow_client.get_achievements_index().await;
    }

    #[test]
    fn test_builder_defaults() {
        let builder = BlizzardAPIClientBuilder::new();
        assert_eq!(builder.region, Some(Region::US));
        assert_eq!(builder.locale, Some(Locale::en_US));
        assert_eq!(builder.timeout, Some(Duration::from_secs(30)));
        assert_eq!(builder.user_agent, Some("ironforge/0.2.0".to_string()));
        assert_eq!(builder.connection_pool_size, Some(10));
    }

    #[test]
    fn test_builder_validation() {
        let result = BlizzardAPIClientBuilder::new().build();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BlizzardAPIClientError::ConfigurationError(_)
        ));

        let result = BlizzardAPIClientBuilder::new()
            .client_id("test_id")
            .client_secret("test_secret")
            .region(Region::US)
            .locale(Locale::en_US)
            .build();
        assert!(result.is_ok());
    }
}
