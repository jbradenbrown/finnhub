//! Corporate actions and filings models.

use serde::{Deserialize, Serialize};

/// Dividend data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dividend {
    /// Symbol.
    pub symbol: String,
    /// Dividend amount.
    pub amount: f64,
    /// Adjusted dividend amount.
    #[serde(rename = "adjustedAmount")]
    pub adjusted_amount: f64,
    /// Currency.
    pub currency: String,
    /// Declaration date.
    #[serde(rename = "declarationDate")]
    pub declaration_date: String,
    /// Ex-dividend date.
    #[serde(rename = "exDividendDate")]
    pub ex_dividend_date: Option<String>,
    /// Frequency.
    pub freq: Option<String>,
    /// Payment date.
    #[serde(rename = "payDate")]
    pub pay_date: String,
    /// Record date.
    #[serde(rename = "recordDate")]
    pub record_date: String,
}

/// Stock split data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSplit {
    /// Symbol.
    pub symbol: String,
    /// Split date.
    pub date: String,
    /// Split from factor.
    #[serde(rename = "fromFactor")]
    pub from_factor: f64,
    /// Split to factor.
    #[serde(rename = "toFactor")]
    pub to_factor: f64,
}

/// Dividends v2 data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendsV2 {
    /// Symbol.
    pub symbol: String,
    /// Array of dividend data.
    pub data: Vec<DividendV2>,
}

/// Dividend v2 information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendV2 {
    /// Ex-dividend date.
    #[serde(rename = "exDate")]
    pub ex_date: String,
    /// Dividend amount.
    pub amount: f64,
}

/// Single symbol-change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolChangeEvent {
    /// Effective date.
    pub at_date: String,
    /// New symbol.
    pub new_symbol: String,
    /// Old symbol.
    pub old_symbol: String,
}

/// Symbol change response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolChange {
    /// Array of symbol-change events.
    pub data: Vec<SymbolChangeEvent>,
    /// From date.
    pub from_date: String,
    /// To date.
    pub to_date: String,
}

/// Single ISIN-change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsinChangeEvent {
    /// Effective date.
    pub at_date: String,
    /// New ISIN.
    pub new_isin: String,
    /// Old ISIN.
    pub old_isin: String,
}

/// ISIN change response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsinChange {
    /// Array of ISIN-change events.
    pub data: Vec<IsinChangeEvent>,
    /// From date.
    pub from_date: String,
    /// To date.
    pub to_date: String,
}
