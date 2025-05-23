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