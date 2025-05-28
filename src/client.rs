//! Main Finnhub client implementation.

use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

use crate::{
    auth::{Auth, AuthMethod},
    endpoints::{CryptoEndpoints, ForexEndpoints, StockEndpoints},
    error::{Error, Result},
    rate_limiter::RateLimiter,
};

const DEFAULT_BASE_URL: &str = "https://finnhub.io/api/v1";
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Configuration for the Finnhub client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the API.
    pub base_url: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Authentication method.
    pub auth_method: AuthMethod,
    /// Custom rate limit (requests per second).
    pub rate_limit: Option<u32>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            auth_method: AuthMethod::default(),
            rate_limit: None,
        }
    }
}

/// Main client for interacting with the Finnhub API.
#[derive(Clone)]
pub struct FinnhubClient {
    http_client: HttpClient,
    auth: Arc<Auth>,
    rate_limiter: Arc<RateLimiter>,
    base_url: Url,
}

impl FinnhubClient {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(api_key, ClientConfig::default())
    }

    /// Create a new client with custom configuration.
    pub fn with_config(api_key: impl Into<String>, config: ClientConfig) -> Self {
        let auth = Auth::with_method(api_key, config.auth_method);

        let mut builder =
            HttpClient::builder().timeout(std::time::Duration::from_secs(config.timeout_secs));

        // Only add headers if using header authentication
        if matches!(config.auth_method, AuthMethod::Header) {
            builder = builder.default_headers(auth.headers());
        }

        let http_client = builder.build().expect("Failed to build HTTP client");

        let rate_limit = config.rate_limit.unwrap_or(30);
        let rate_limiter = RateLimiter::new(rate_limit, rate_limit);

        let base_url = Url::parse(&config.base_url).expect("Invalid base URL");

        Self {
            http_client,
            auth: Arc::new(auth),
            rate_limiter: Arc::new(rate_limiter),
            base_url,
        }
    }

    /// Get stock market endpoints.
    pub fn stock(&self) -> StockEndpoints<'_> {
        StockEndpoints::new(self)
    }

    /// Get forex market endpoints.
    pub fn forex(&self) -> ForexEndpoints<'_> {
        ForexEndpoints::new(self)
    }

    /// Get cryptocurrency endpoints.
    pub fn crypto(&self) -> CryptoEndpoints<'_> {
        CryptoEndpoints::new(self)
    }

    /// Make a GET request to the API.
    pub(crate) async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.rate_limiter.acquire().await?;

        let mut url = self.base_url.clone();
        url.set_path(&format!("/api/v1{}", endpoint));

        // Apply auth to URL if using URL parameter method
        self.auth.apply_to_url(&mut url);

        let response = self.http_client.get(url).send().await?;

        self.handle_response(response).await
    }

    /// Handle API response.
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(Into::into)
        } else {
            match status.as_u16() {
                401 => Err(Error::Unauthorized),
                429 => {
                    let retry_after = response
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(60);

                    Err(Error::RateLimitExceeded { retry_after })
                }
                _ => {
                    let message = response
                        .text()
                        .await
                        .unwrap_or_else(|_| format!("HTTP error {}", status.as_u16()));

                    Err(Error::ApiError {
                        status: status.as_u16(),
                        message,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = FinnhubClient::new("test-api-key");
        assert!(client.auth.api_key() == "test-api-key");
    }
}
