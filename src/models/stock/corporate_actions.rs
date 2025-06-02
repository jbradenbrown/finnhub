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
