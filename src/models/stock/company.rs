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

/// Premium company profile (richer than `CompanyProfile`).
///
/// Returned by `/stock/profile`. Includes detailed industry classification
/// (GICS, NAICS), insider/institutional ownership, IR URL, and more.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfilePremium {
    /// Company address.
    pub address: Option<String>,
    /// Other company name aliases.
    pub alias: Option<serde_json::Value>,
    /// Headquarter city.
    pub city: Option<String>,
    /// Country of headquarter.
    pub country: Option<String>,
    /// Reporting currency.
    pub currency: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// Long company description.
    pub description: Option<String>,
    /// Total employee count.
    pub employee_total: Option<i64>,
    /// Currency used in analyst estimates.
    pub estimate_currency: Option<String>,
    /// Exchange name.
    pub exchange: Option<String>,
    /// Finnhub industry classification.
    pub finnhub_industry: Option<String>,
    /// Floating share count (millions).
    pub floating_share: Option<f64>,
    /// Reporting frequency (`q` or `a`).
    pub fundamental_freq: Option<String>,
    /// GICS group.
    pub ggroup: Option<String>,
    /// GICS industry.
    pub gind: Option<String>,
    /// GICS sector.
    pub gsector: Option<String>,
    /// GICS sub-industry.
    pub gsubind: Option<String>,
    /// Insider ownership (fraction, 0..1).
    pub insider_ownership: Option<f64>,
    /// Institutional ownership (percent).
    pub institution_ownership: Option<f64>,
    /// IPO date.
    pub ipo: Option<String>,
    /// Investor relations URL.
    pub ir_url: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// Legal Entity Identifier.
    pub lei: Option<String>,
    /// Logo URL.
    pub logo: Option<String>,
    /// Currency of `marketCapitalization`.
    pub market_cap_currency: Option<String>,
    /// Market capitalization in reporting currency.
    pub market_capitalization: Option<f64>,
    /// Market capitalization in USD.
    #[serde(rename = "marketcapUSD")]
    pub marketcap_usd: Option<f64>,
    /// NAICS industry.
    pub naics: Option<String>,
    /// NAICS national industry.
    pub naics_national_industry: Option<String>,
    /// NAICS sector.
    pub naics_sector: Option<String>,
    /// NAICS subsector.
    pub naics_subsector: Option<String>,
    /// Company name.
    pub name: Option<String>,
    /// Phone number.
    pub phone: Option<String>,
    /// SEDOL.
    pub sedol: Option<String>,
    /// Total shares outstanding (millions).
    pub share_outstanding: Option<f64>,
    /// State of headquarter.
    pub state: Option<String>,
    /// Ticker.
    pub ticker: Option<String>,
    /// Share of revenue from US operations.
    #[serde(rename = "usShare")]
    pub us_share: Option<f64>,
    /// Website URL.
    pub weburl: Option<String>,
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
