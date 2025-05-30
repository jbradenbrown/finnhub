//! Company information endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{CompanyProfile, Symbol},
};

/// Company information endpoints.
pub struct CompanyEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> CompanyEndpoints<'a> {
    /// Create a new company endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get company profile.
    pub async fn profile(&self, symbol: &str) -> Result<CompanyProfile> {
        self.client
            .get(&format!("/stock/profile2?symbol={}", symbol))
            .await
    }

    /// Get company peers.
    ///
    /// Returns a list of peers operating in the same country and sector/industry.
    pub async fn peers(&self, symbol: &str, grouping: Option<&str>) -> Result<Vec<String>> {
        let url = if let Some(grouping) = grouping {
            format!("/stock/peers?symbol={}&grouping={}", symbol, grouping)
        } else {
            format!("/stock/peers?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get list of supported stocks.
    ///
    /// List all supported stocks for a given exchange.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<Symbol>> {
        self.client
            .get(&format!("/stock/symbol?exchange={}", exchange))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientConfig, FinnhubClient, RateLimitStrategy};

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY").unwrap_or_else(|_| "test_key".to_string());

        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_company_profile() {
        let client = test_client().await;
        let result = client.stock().company_profile("AAPL").await;
        assert!(
            result.is_ok(),
            "Failed to get company profile: {:?}",
            result.err()
        );

        let profile = result.unwrap();
        assert!(profile.name.is_some());
        assert_eq!(profile.ticker.as_deref(), Some("AAPL"));
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_peers() {
        let client = test_client().await;
        let result = client.stock().peers("AAPL", None).await;
        assert!(result.is_ok(), "Failed to get peers: {:?}", result.err());

        let peers = result.unwrap();
        assert!(!peers.is_empty());
        assert!(peers.len() <= 10); // Usually returns up to 10 peers
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_symbols() {
        let client = test_client().await;
        let result = client.stock().symbols("US").await;
        assert!(result.is_ok(), "Failed to get symbols: {:?}", result.err());

        let symbols = result.unwrap();
        assert!(!symbols.is_empty());
        // Check that symbols have the expected fields
        if let Some(symbol) = symbols.first() {
            assert!(!symbol.symbol.is_empty());
            assert!(!symbol.description.is_empty());
        }
    }
}
