//! Error types for the Finnhub client.

use thiserror::Error;

/// Result type alias for Finnhub operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Finnhub client.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded: please retry after {retry_after} seconds")]
    RateLimitExceeded {
        /// Number of seconds to wait before retrying.
        retry_after: u64,
    },

    /// Invalid API key or unauthorized request.
    #[error("Unauthorized: invalid API key")]
    Unauthorized,

    /// API returned an error response.
    #[error("API error (status {status}): {message}")]
    ApiError {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
    },

    /// Failed to deserialize response.
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    /// Invalid parameter provided.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// WebSocket error.
    #[cfg(feature = "websocket")]
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// URL parsing error.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Timeout error.
    #[error("Request timeout")]
    Timeout,

    /// Generic error for unexpected cases.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Create a new invalid parameter error.
    pub fn invalid_parameter(param: impl Into<String>) -> Self {
        Self::InvalidParameter(param.into())
    }

    /// Create a new internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimitExceeded { .. } | Self::Timeout | Self::Http(_)
        )
    }

    /// Get the retry delay in seconds if applicable.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::RateLimitExceeded { retry_after } => Some(*retry_after),
            Self::Timeout => Some(5), // Default retry after 5 seconds for timeout
            _ => None,
        }
    }
}
