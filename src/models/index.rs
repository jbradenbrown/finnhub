//! Index models.

use serde::{Deserialize, Serialize};

/// Index constituents data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicesConstituents {
    /// Index symbol.
    pub symbol: String,
    /// List of constituent symbols.
    pub constituents: Vec<String>,
    /// Detailed breakdown of constituents.
    #[serde(rename = "constituentsBreakdown")]
    pub constituents_breakdown: Vec<ConstituentDetails>,
}

/// Constituent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstituentDetails {
    /// Symbol.
    pub symbol: String,
    /// Company name.
    pub name: String,
    /// CUSIP.
    pub cusip: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// Share class FIGI.
    #[serde(rename = "shareClassFIGI")]
    pub share_class_figi: Option<String>,
    /// Weight in the index.
    pub weight: Option<f64>,
}

/// Historical index constituents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicesHistoricalConstituents {
    /// Index symbol.
    pub symbol: String,
    /// Historical data array.
    #[serde(rename = "historicalConstituents")]
    pub historical_constituents: Vec<HistoricalConstituent>,
}

/// Historical constituent data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalConstituent {
    /// Symbol.
    pub symbol: String,
    /// Action (added or removed).
    pub action: String,
    /// Date of action.
    pub date: String,
    /// Company name.
    pub name: Option<String>,
}
