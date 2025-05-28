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
