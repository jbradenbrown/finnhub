//! Insider trading endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{InsiderSentimentData, InsiderTransactions},
};

/// Insider trading endpoints.
pub struct InsiderEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> InsiderEndpoints<'a> {
    /// Create a new insider endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get insider transactions.
    ///
    /// Returns insider transactions for the last 3 months.
    pub async fn transactions(&self, symbol: &str) -> Result<InsiderTransactions> {
        self.client
            .get(&format!("/stock/insider-transactions?symbol={}", symbol))
            .await
    }

    /// Get insider sentiment data.
    ///
    /// Returns aggregated insider trading sentiment by month.
    pub async fn sentiment(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<InsiderSentimentData> {
        self.client
            .get(&format!(
                "/stock/insider-sentiment?symbol={}&from={}&to={}",
                symbol, from, to
            ))
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
    async fn test_insider_transactions() {
        let client = test_client().await;
        let result = client.stock().insider_transactions("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get insider transactions: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_insider_sentiment() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client.stock().insider_sentiment("MSFT", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get insider sentiment: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_insider_sentiment_date_range() {
        let client = test_client().await;
        let from = "2024-01-01";
        let to = "2024-06-30";
        let result = client.stock().insider_sentiment("GOOGL", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get insider sentiment with date range: {:?}",
            result.err()
        );
    }
}
