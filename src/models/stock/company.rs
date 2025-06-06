//! Company information models.

use serde::{Deserialize, Serialize};

/// Company profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyProfile {
    /// Country of company's headquarter.
    pub country: Option<String>,
    /// Currency used in company filings.
    pub currency: Option<String>,
    /// Exchange name.
    pub exchange: Option<String>,
    /// Company name.
    pub name: Option<String>,
    /// Company ticker symbol.
    pub ticker: Option<String>,
    /// Company IPO date.
    pub ipo: Option<String>,
    /// Market capitalization.
    #[serde(rename = "marketCapitalization")]
    pub market_capitalization: Option<f64>,
    /// Number of shares outstanding.
    #[serde(rename = "shareOutstanding")]
    pub share_outstanding: Option<f64>,
    /// Company logo URL.
    pub logo: Option<String>,
    /// Company phone number.
    pub phone: Option<String>,
    /// Company website URL.
    pub weburl: Option<String>,
    /// Finnhub industry classification.
    #[serde(rename = "finnhubIndustry")]
    pub finnhub_industry: Option<String>,
}

/// Stock symbol information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Symbol description.
    pub description: String,
    /// Display symbol.
    #[serde(rename = "displaySymbol")]
    pub display_symbol: String,
    /// Symbol ticker.
    pub symbol: String,
    /// Security type.
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
    /// Primary exchange.
    pub mic: Option<String>,
    /// FIGI identifier.
    pub figi: Option<String>,
    /// Share class FIGI.
    #[serde(rename = "shareClassFIGI")]
    pub share_class_figi: Option<String>,
    /// Currency.
    pub currency: Option<String>,
}
