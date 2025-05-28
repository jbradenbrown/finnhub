//! Forex-related data models.

use serde::{Deserialize, Serialize};

/// Forex symbol information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexSymbol {
    /// Currency pair description.
    pub description: String,
    /// Display symbol.
    pub display_symbol: String,
    /// Symbol.
    pub symbol: String,
}

/// Forex rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexRates {
    /// Base currency.
    pub base: String,
    /// Quote data with currency codes as keys.
    pub quote: std::collections::HashMap<String, f64>,
}

/// Forex candles (OHLCV) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexCandles {
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
