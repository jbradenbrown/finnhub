//! Stock-related data models.

use serde::{Deserialize, Serialize};
use super::common::Currency;

/// Stock quote data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Current price.
    #[serde(rename = "c")]
    pub current_price: f64,
    /// Change.
    #[serde(rename = "d")]
    pub change: f64,
    /// Percent change.
    #[serde(rename = "dp")]
    pub percent_change: f64,
    /// High price of the day.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price of the day.
    #[serde(rename = "l")]
    pub low: f64,
    /// Open price of the day.
    #[serde(rename = "o")]
    pub open: f64,
    /// Previous close price.
    #[serde(rename = "pc")]
    pub previous_close: f64,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: i64,
}

/// Company profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
    /// Country of company's headquarters.
    pub country: String,
    /// Currency used in company filings.
    pub currency: Currency,
    /// Listed exchange.
    pub exchange: String,
    /// Finnhub industry classification.
    pub finnhub_industry: String,
    /// IPO date.
    pub ipo: Option<String>,
    /// Company logo URL.
    pub logo: Option<String>,
    /// Market capitalization.
    pub market_capitalization: f64,
    /// Company name.
    pub name: String,
    /// Company phone number.
    pub phone: Option<String>,
    /// Number of employees.
    pub share_outstanding: f64,
    /// Company symbol/ticker.
    pub ticker: String,
    /// Company website.
    pub weburl: Option<String>,
}

/// Stock symbol information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Symbol description.
    pub description: String,
    /// Display symbol.
    pub display_symbol: String,
    /// Symbol/ticker.
    pub symbol: String,
    /// Security type.
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
    /// Currency.
    pub currency: Option<Currency>,
    /// FIGI identifier.
    pub figi: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// MIC code.
    pub mic: Option<String>,
}