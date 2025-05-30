//! Economic data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::economic::{EconomicCode, EconomicData},
};

/// Economic data API endpoints.
pub struct EconomicEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> EconomicEndpoints<'a> {
    /// Create a new economic endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get economic data.
    ///
    /// Returns economic data for a specific indicator.
    ///
    /// # Arguments
    /// * `code` - Economic indicator code (e.g., "MA-USA-656880")
    pub async fn data(&self, code: &str) -> Result<EconomicData> {
        self.client
            .get(&format!("/economic?code={}", code))
            .await
    }

    /// Get list of economic indicator codes.
    ///
    /// Returns all available economic indicator codes.
    pub async fn codes(&self) -> Result<Vec<EconomicCode>> {
        self.client.get("/economic/code").await
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientConfig, FinnhubClient, RateLimitStrategy};

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY")
            .unwrap_or_else(|_| "test_key".to_string());
        
        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_codes() {
        let client = test_client().await;
        let result = client.economic().codes().await;
        assert!(result.is_ok(), "Failed to get economic codes: {:?}", result.err());
        
        let codes = result.unwrap();
        assert!(!codes.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_data() {
        let client = test_client().await;
        let result = client.economic().data("MA-USA-656880").await;
        assert!(result.is_ok(), "Failed to get economic data: {:?}", result.err());
    }
}