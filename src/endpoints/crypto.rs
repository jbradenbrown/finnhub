//! Cryptocurrency endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::{crypto::*, stock::CandleResolution},
};

/// Crypto-related API endpoints.
pub struct CryptoEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> CryptoEndpoints<'a> {
    /// Create a new crypto endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get supported crypto exchanges.
    pub async fn exchanges(&self) -> Result<Vec<CryptoExchange>> {
        self.client.get("/crypto/exchange").await
    }

    /// Get supported crypto symbols.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<CryptoSymbol>> {
        self.client
            .get(&format!("/crypto/symbol?exchange={}", exchange))
            .await
    }

    /// Get crypto candlestick data.
    ///
    /// Get OHLCV data for crypto symbols.
    pub async fn candles(
        &self,
        symbol: &str,
        resolution: CandleResolution,
        from: i64,
        to: i64,
    ) -> Result<CryptoCandles> {
        self.client
            .get(&format!(
                "/crypto/candle?symbol={}&resolution={}&from={}&to={}",
                symbol, resolution, from, to
            ))
            .await
    }

    /// Get crypto profile data.
    ///
    /// Get general information about a cryptocurrency.
    pub async fn profile(&self, symbol: &str) -> Result<CryptoProfile> {
        self.client
            .get(&format!("/crypto/profile?symbol={}", symbol))
            .await
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
    async fn test_exchanges() {
        let client = test_client().await;
        let result = client.crypto().exchanges().await;
        assert!(
            result.is_ok(),
            "Failed to get crypto exchanges: {:?}",
            result.err()
        );

        let exchanges = result.unwrap();
        assert!(!exchanges.is_empty());

        // Check that exchanges have the expected format
        if let Some(exchange) = exchanges.first() {
            assert!(!exchange.code.is_empty());
            assert!(!exchange.name.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_symbols() {
        let client = test_client().await;
        let result = client.crypto().symbols("BINANCE").await;
        assert!(
            result.is_ok(),
            "Failed to get crypto symbols: {:?}",
            result.err()
        );

        let symbols = result.unwrap();
        assert!(!symbols.is_empty());

        // Check that symbols have the expected format
        if let Some(symbol) = symbols.first() {
            assert!(!symbol.description.is_empty());
            assert!(!symbol.symbol.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_candles() {
        let client = test_client().await;
        let from = chrono::Utc::now().timestamp() - 86400; // 1 day ago
        let to = chrono::Utc::now().timestamp();

        let result = client
            .crypto()
            .candles("BINANCE:BTCUSDT", CandleResolution::SixtyMinutes, from, to)
            .await;
        assert!(
            result.is_ok(),
            "Failed to get crypto candles: {:?}",
            result.err()
        );

        let candles = result.unwrap();
        assert_eq!(candles.status, "ok");
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_profile() {
        let client = test_client().await;
        let result = client.crypto().profile("BTC").await;
        assert!(
            result.is_ok(),
            "Failed to get crypto profile: {:?}",
            result.err()
        );
    }
}
