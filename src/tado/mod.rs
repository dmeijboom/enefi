mod client_info;
mod oauth;

use std::sync::Arc;

pub use client_info::{get_client_info, ClientInfo};
pub use oauth::{GrantType, RequestTokenRequest, RequestTokenResponse};

pub struct Endpoints {
    pub oauth: String,
}

#[derive(Clone)]
pub struct Client {
    endpoints: Arc<Endpoints>,
    http: reqwest::Client,
}

impl Client {
    pub fn new(endpoints: Endpoints) -> Self {
        Self {
            endpoints: Arc::new(endpoints),
            http: reqwest::Client::new(),
        }
    }

    pub fn oauth(&self) -> oauth::Client {
        return oauth::Client::new(Arc::clone(&self.endpoints), self.http.clone());
    }
}
