//! Authentication handling for the Finnhub API.

use reqwest::header::{HeaderMap, HeaderValue};

/// Authentication method for API requests.
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// Use API key as URL parameter.
    UrlParameter,
    /// Use API key in request header.
    Header,
}

impl Default for AuthMethod {
    fn default() -> Self {
        Self::Header // Prefer header method for security
    }
}

/// Authentication configuration.
#[derive(Debug, Clone)]
pub struct Auth {
    api_key: String,
    method: AuthMethod,
}

impl Auth {
    /// Create a new authentication configuration.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            method: AuthMethod::default(),
        }
    }

    /// Create authentication with specific method.
    pub fn with_method(api_key: impl Into<String>, method: AuthMethod) -> Self {
        Self {
            api_key: api_key.into(),
            method,
        }
    }

    /// Get the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the authentication method.
    pub fn method(&self) -> &AuthMethod {
        &self.method
    }

    /// Apply authentication to a URL.
    pub fn apply_to_url(&self, url: &mut url::Url) {
        if matches!(self.method, AuthMethod::UrlParameter) {
            url.query_pairs_mut().append_pair("token", &self.api_key);
        }
    }

    /// Create headers for authentication.
    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if matches!(self.method, AuthMethod::Header) {
            if let Ok(value) = HeaderValue::from_str(&self.api_key) {
                headers.insert("X-Finnhub-Token", value);
            }
        }
        headers
    }
}
