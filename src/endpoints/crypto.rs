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
