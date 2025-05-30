//! Technical analysis scanner endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::scanner::{AggregateIndicators, PatternRecognition, SupportResistance},
};

/// Scanner/Technical Analysis endpoints.
pub struct ScannerEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> ScannerEndpoints<'a> {
    /// Create a new instance of scanner endpoints.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Run pattern recognition algorithm on a symbol.
    /// Supports double top/bottom, triple top/bottom, head and shoulders, triangle, wedge, channel, flag, and candlestick patterns.
    pub async fn pattern_recognition(
        &self,
        symbol: &str,
        resolution: &str,
    ) -> Result<PatternRecognition> {
        self.client
            .get(&format!(
                "/scan/pattern?symbol={}&resolution={}",
                symbol, resolution
            ))
            .await
    }

    /// Get support and resistance levels for a symbol.
    pub async fn support_resistance(
        &self,
        symbol: &str,
        resolution: &str,
    ) -> Result<SupportResistance> {
        self.client
            .get(&format!(
                "/scan/support-resistance?symbol={}&resolution={}",
                symbol, resolution
            ))
            .await
    }

    /// Get aggregate signal of multiple technical indicators.
    /// Includes MACD, RSI, Moving Average and more.
    pub async fn aggregate_indicators(
        &self,
        symbol: &str,
        resolution: &str,
    ) -> Result<AggregateIndicators> {
        self.client
            .get(&format!(
                "/scan/technical-indicator?symbol={}&resolution={}",
                symbol, resolution
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
    async fn test_pattern_recognition() {
        let client = test_client().await;
        let result = client.scanner().pattern_recognition("AAPL", "D").await;
        assert!(
            result.is_ok(),
            "Failed to get pattern recognition: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_support_resistance() {
        let client = test_client().await;
        let result = client.scanner().support_resistance("AAPL", "D").await;
        assert!(
            result.is_ok(),
            "Failed to get support resistance: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_aggregate_indicators() {
        let client = test_client().await;
        let result = client.scanner().aggregate_indicators("AAPL", "D").await;
        assert!(
            result.is_ok(),
            "Failed to get aggregate indicators: {:?}",
            result.err()
        );
    }
}
