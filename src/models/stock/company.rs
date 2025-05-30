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

/// Supply chain relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainRelationship {
    /// Symbol.
    pub symbol: Option<String>,
    /// Company name.
    pub name: Option<String>,
    /// Country.
    pub country: Option<String>,
    /// 1-tier supplier.
    #[serde(rename = "oneMonthCorrelation")]
    pub one_month_correlation: Option<f64>,
    /// 1-year correlation.
    #[serde(rename = "oneYearCorrelation")]
    pub one_year_correlation: Option<f64>,
    /// 6-month correlation.
    #[serde(rename = "sixMonthCorrelation")]
    pub six_month_correlation: Option<f64>,
    /// 3-month correlation.
    #[serde(rename = "threeMonthCorrelation")]
    pub three_month_correlation: Option<f64>,
    /// 2-week correlation.
    #[serde(rename = "twoWeekCorrelation")]
    pub two_week_correlation: Option<f64>,
    /// 2-year correlation.
    #[serde(rename = "twoYearCorrelation")]
    pub two_year_correlation: Option<f64>,
}

/// Supply chain data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainData {
    /// Company symbol.
    pub symbol: String,
    /// List of suppliers.
    pub data: Vec<SupplyChainRelationship>,
}
