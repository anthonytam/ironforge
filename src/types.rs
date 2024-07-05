use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct BlizzardAccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub sub: String,
    pub scope: Option<String>
}