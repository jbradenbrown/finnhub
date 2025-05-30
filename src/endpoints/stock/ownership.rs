//! Ownership data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{FundOwnership, OwnershipData},
};

/// Ownership data endpoints.
pub struct OwnershipEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> OwnershipEndpoints<'a> {
    /// Create a new ownership endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get company ownership data.
    ///
    /// Returns a list of company shareholders/owners.
    pub async fn institutional(&self, symbol: &str, limit: Option<i64>) -> Result<OwnershipData> {
        let url = if let Some(limit) = limit {
            format!("/stock/ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get fund ownership.
    ///
    /// Get a list of funds that hold shares of a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `limit` - Limit number of results (optional)
    pub async fn fund(&self, symbol: &str, limit: Option<i64>) -> Result<FundOwnership> {
        let url = if let Some(limit) = limit {
            format!("/stock/fund-ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/fund-ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
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
    async fn test_institutional_ownership() {
        let client = test_client().await;
        let result = client.stock().ownership("AAPL", None).await;
        
        assert!(result.is_ok(), "Failed to get institutional ownership: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_ownership_with_limit() {
        let client = test_client().await;
        let limit = 10;
        let result = client.stock().ownership("MSFT", Some(limit)).await;
        
        assert!(result.is_ok(), "Failed to get institutional ownership with limit: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_fund_ownership() {
        let client = test_client().await;
        let result = client.stock().fund_ownership("AAPL", None).await;
        
        assert!(result.is_ok(), "Failed to get fund ownership: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_fund_ownership_with_limit() {
        let client = test_client().await;
        let limit = 5;
        let result = client.stock().fund_ownership("GOOGL", Some(limit)).await;
        
        assert!(result.is_ok(), "Failed to get fund ownership with limit: {:?}", result.err());
    }
}