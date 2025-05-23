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