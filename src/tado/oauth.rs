use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::Endpoints;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Password,
    RefreshToken,
}

#[derive(Debug, Serialize)]
pub struct RequestTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: GrantType,
    pub scope: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub jti: String,
}

pub struct Client {
    endpoints: Arc<Endpoints>,
    http: reqwest::Client,
}

impl Client {
    pub fn new(endpoints: Arc<Endpoints>, http: reqwest::Client) -> Self {
        Self { endpoints, http }
    }

    pub async fn request_token(
        &self,
        request: &RequestTokenRequest,
    ) -> Result<RequestTokenResponse> {
        let response = self
            .http
            .post([self.endpoints.oauth.as_str(), "/token"].concat())
            .form(request)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
