//! Option chain endpoint.

use crate::{client::FinnhubClient, error::Result, models::stock::OptionChain};

/// Option chain endpoint.
pub struct OptionsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> OptionsEndpoints<'a> {
    /// Create a new options endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get the option chain for a symbol.
    ///
    /// Returns one entry per expiration date, each containing all listed
    /// call and put contracts with pricing, volume, open interest, implied
    /// volatility, and Greeks.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn option_chain(&self, symbol: &str) -> Result<OptionChain> {
        self.client
            .get(&format!("/stock/option-chain?symbol={}", symbol))
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
    async fn test_option_chain() {
        let client = test_client().await;
        let result = client.stock().option_chain("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get option chain: {:?}",
            result.err()
        );

        let chain = result.unwrap();
        assert_eq!(chain.code, "AAPL");
        assert!(!chain.data.is_empty(), "Expected at least one expiration");

        let first = &chain.data[0];
        assert!(!first.expiration_date.is_empty());
        // At least one side should usually have contracts.
        let has_contracts =
            !first.options.call.is_empty() || !first.options.put.is_empty();
        assert!(has_contracts, "Expected at least one CALL or PUT");

        // Spot-check that contract fields parse.
        if let Some(c) = first.options.call.first() {
            assert_eq!(c.option_type, "CALL");
            assert!(c.strike > 0.0);
            assert!(!c.contract_name.is_empty());
        }
    }
}
