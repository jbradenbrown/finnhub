//! Price-related stock endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        BidAsk, CandleResolution, PriceMetrics, Quote, StockCandles, TickData,
    },
};

/// Price-related endpoints for stocks.
pub struct PriceEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> PriceEndpoints<'a> {
    /// Create a new price endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get real-time quote data.
    pub async fn quote(&self, symbol: &str) -> Result<Quote> {
        self.client.get(&format!("/quote?symbol={}", symbol)).await
    }

    /// Get candlestick data (OHLCV) for stocks.
    ///
    /// Daily data will be adjusted for splits. Intraday data will remain unadjusted.
    /// Only 1 month of intraday data will be returned at a time.
    pub async fn candles(
        &self,
        symbol: &str,
        resolution: CandleResolution,
        from: i64,
        to: i64,
    ) -> Result<StockCandles> {
        self.client
            .get(&format!(
                "/stock/candle?symbol={}&resolution={}&from={}&to={}",
                symbol, resolution, from, to
            ))
            .await
    }

    /// Get last bid-ask data.
    ///
    /// Returns the last bid and ask prices with volumes for US stocks.
    pub async fn bid_ask(&self, symbol: &str) -> Result<BidAsk> {
        self.client
            .get(&format!("/stock/bidask?symbol={}", symbol))
            .await
    }

    /// Get historical tick data.
    ///
    /// Returns historical tick data for global exchanges.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `date` - Date in YYYY-MM-DD format
    /// * `limit` - Limit number of ticks returned (max 25000)
    /// * `skip` - Number of ticks to skip
    pub async fn tick_data(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<TickData> {
        self.client
            .get(&format!(
                "/stock/tick?symbol={}&date={}&limit={}&skip={}",
                symbol, date, limit, skip
            ))
            .await
    }

    /// Get price metrics.
    ///
    /// Get advanced price performance metrics for a stock.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn price_metrics(&self, symbol: &str) -> Result<PriceMetrics> {
        self.client
            .get(&format!("/stock/price-metric?symbol={}", symbol))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_quote() {
        let client = test_client().await;
        let result = client.stock().quote("AAPL").await;
        assert!(result.is_ok(), "Failed to get quote: {:?}", result.err());
        
        let quote = result.unwrap();
        assert!(quote.current_price > 0.0);
        assert!(quote.high >= quote.low);
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_candles() {
        let client = test_client().await;
        let from = chrono::Utc::now().timestamp() - 86400 * 7; // 7 days ago
        let to = chrono::Utc::now().timestamp();
        
        let result = client.stock().candles("AAPL", CandleResolution::Daily, from, to).await;
        assert!(result.is_ok(), "Failed to get candles: {:?}", result.err());
        
        let candles = result.unwrap();
        assert_eq!(candles.status, "ok");
        assert!(!candles.close.is_empty());
        assert_eq!(candles.close.len(), candles.open.len());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_bid_ask() {
        let client = test_client().await;
        let result = client.stock().bid_ask("AAPL").await;
        // Bid-ask may not always be available
        assert!(result.is_ok(), "Failed to get bid-ask: {:?}", result.err());
    }
}