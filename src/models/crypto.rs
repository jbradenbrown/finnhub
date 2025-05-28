//! Cryptocurrency-related data models.

use serde::{Deserialize, Serialize};

/// Crypto symbol information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoSymbol {
    /// Symbol description.
    pub description: String,
    /// Display symbol.
    pub display_symbol: String,
    /// Symbol.
    pub symbol: String,
}

/// Crypto exchange information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoExchange {
    /// Exchange code.
    pub code: String,
    /// Exchange name.
    pub name: String,
}

/// Crypto candles (OHLCV) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoCandles {
    /// List of open prices.
    #[serde(rename = "o")]
    pub open: Vec<f64>,
    /// List of high prices.
    #[serde(rename = "h")]
    pub high: Vec<f64>,
    /// List of low prices.
    #[serde(rename = "l")]
    pub low: Vec<f64>,
    /// List of close prices.
    #[serde(rename = "c")]
    pub close: Vec<f64>,
    /// List of volume data.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,
    /// List of timestamps.
    #[serde(rename = "t")]
    pub timestamp: Vec<i64>,
    /// Status of the response.
    #[serde(rename = "s")]
    pub status: String,
}

/// Crypto profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoProfile {
    /// Symbol.
    pub symbol: String,
    /// Name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Website.
    pub website: Option<String>,
    /// Market cap.
    pub market_cap: Option<f64>,
    /// Logo URL.
    pub logo: Option<String>,
}
