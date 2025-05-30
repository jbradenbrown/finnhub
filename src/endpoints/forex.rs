//! Forex market endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::{forex::*, stock::CandleResolution},
};

/// Forex-related API endpoints.
pub struct ForexEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> ForexEndpoints<'a> {
    /// Create a new forex endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get supported forex symbols.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<ForexSymbol>> {
        self.client
            .get(&format!("/forex/symbol?exchange={}", exchange))
            .await
    }

    /// Get forex candlestick data.
    ///
    /// Get OHLCV data for forex symbols.
    pub async fn candles(
        &self,
        symbol: &str,
        resolution: CandleResolution,
        from: i64,
        to: i64,
    ) -> Result<ForexCandles> {
        self.client
            .get(&format!(
                "/forex/candle?symbol={}&resolution={}&from={}&to={}",
                symbol, resolution, from, to
            ))
            .await
    }

    /// Get forex exchange rates.
    ///
    /// Get real-time exchange rates for forex pairs.
    pub async fn rates(&self, base: &str) -> Result<ForexRates> {
        self.client
            .get(&format!("/forex/rates?base={}", base))
            .await
    }

    /// Get supported forex exchanges.
    pub async fn exchanges(&self) -> Result<Vec<String>> {
        self.client.get("/forex/exchange").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_forex_symbols() {
        let client = test_client().await;
        let result = client.forex().symbols("OANDA").await;
        assert!(
            result.is_ok(),
            "Failed to get forex symbols: {:?}",
            result.err()
        );

        let symbols = result.unwrap();
        assert!(!symbols.is_empty());

        // Check that symbols have the expected format
        for symbol in &symbols {
            assert!(!symbol.description.is_empty());
            assert!(!symbol.symbol.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_forex_exchanges() {
        let client = test_client().await;
        let result = client.forex().exchanges().await;
        assert!(
            result.is_ok(),
            "Failed to get forex exchanges: {:?}",
            result.err()
        );

        let exchanges = result.unwrap();
        assert!(!exchanges.is_empty());
        assert!(exchanges.contains(&"OANDA".to_string()));
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_forex_rates() {
        let client = test_client().await;
        let result = client.forex().rates("USD").await;
        assert!(
            result.is_ok(),
            "Failed to get forex rates: {:?}",
            result.err()
        );

        let rates = result.unwrap();
        assert_eq!(rates.base, "USD");
        assert!(!rates.quote.is_empty());

        // Check some common currencies
        assert!(rates.quote.contains_key("EUR"));
        assert!(rates.quote.contains_key("GBP"));
        assert!(rates.quote.contains_key("JPY"));

        // All rates should be positive
        for (_, rate) in &rates.quote {
            assert!(*rate > 0.0);
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_forex_candles() {
        let client = test_client().await;
        let from = chrono::Utc::now().timestamp() - 86400 * 7; // 7 days ago
        let to = chrono::Utc::now().timestamp();

        let result = client
            .forex()
            .candles("OANDA:EUR_USD", CandleResolution::Daily, from, to)
            .await;
        assert!(
            result.is_ok(),
            "Failed to get forex candles: {:?}",
            result.err()
        );

        let candles = result.unwrap();
        assert_eq!(candles.status, "ok");
        assert!(!candles.close.is_empty());
        assert_eq!(candles.close.len(), candles.open.len());
        assert_eq!(candles.close.len(), candles.high.len());
        assert_eq!(candles.close.len(), candles.low.len());
    }
}
