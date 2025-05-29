//! ETF-related data models.

use serde::{Deserialize, Serialize};

/// ETF profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETFProfile {
    /// Name.
    pub name: Option<String>,
    /// Asset class.
    #[serde(rename = "assetClass")]
    pub asset_class: Option<String>,
    /// Investment segment.
    #[serde(rename = "investmentSegment")]
    pub investment_segment: Option<String>,
    /// AUM (Assets Under Management).
    pub aum: Option<f64>,
    /// NAV (Net Asset Value).
    pub nav: Option<f64>,
    /// NAV currency.
    #[serde(rename = "navCurrency")]
    pub nav_currency: Option<String>,
    /// Expense ratio.
    #[serde(rename = "expenseRatio")]
    pub expense_ratio: Option<f64>,
    /// Tracking index.
    #[serde(rename = "trackingIndex")]
    pub tracking_index: Option<String>,
    /// ETF issuer.
    #[serde(rename = "etfCompany")]
    pub etf_company: Option<String>,
    /// ETF domicile.
    pub domicile: Option<String>,
    /// Inception date.
    #[serde(rename = "inceptionDate")]
    pub inception_date: Option<String>,
    /// ETF's website.
    pub website: Option<String>,
    /// Logo URL.
    pub logo: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// Price to earnings ratio.
    #[serde(rename = "priceToEarnings")]
    pub price_to_earnings: Option<f64>,
    /// Price to book ratio.
    #[serde(rename = "priceToBook")]
    pub price_to_book: Option<f64>,
    /// 30-day average volume.
    #[serde(rename = "avgVolume")]
    pub avg_volume: Option<f64>,
    /// ETF's description.
    pub description: Option<String>,
    /// Whether the ETF is inverse.
    #[serde(rename = "isInverse")]
    pub is_inverse: Option<bool>,
    /// Whether the ETF is leveraged.
    #[serde(rename = "isLeveraged")]
    pub is_leveraged: Option<bool>,
    /// Leverage factor.
    #[serde(rename = "leverageFactor")]
    pub leverage_factor: Option<f64>,
    /// Dividend yield.
    #[serde(rename = "dividendYield")]
    pub dividend_yield: Option<f64>,
}

/// ETF holding data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETFHolding {
    /// Symbol.
    pub symbol: Option<String>,
    /// Security name.
    pub name: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// Number of shares owned by the ETF.
    pub share: Option<f64>,
    /// Portfolio's percent.
    pub percent: Option<f64>,
    /// Market value.
    pub value: Option<f64>,
    /// Asset type (Equity, ETP, Fund, Bond, Other).
    #[serde(rename = "assetType")]
    pub asset_type: Option<String>,
}

/// ETF holdings response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETFHoldings {
    /// ETF symbol.
    pub symbol: String,
    /// As of date.
    #[serde(rename = "atDate")]
    pub at_date: Option<String>,
    /// Array of holdings.
    pub holdings: Vec<ETFHolding>,
}

/// ETF country exposure data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryExposure {
    /// Country name.
    pub country: String,
    /// Exposure percentage.
    pub exposure: f64,
}

/// ETF country exposure response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETFCountryExposure {
    /// ETF symbol.
    pub symbol: String,
    /// Array of country exposures.
    #[serde(rename = "countryExposure")]
    pub country_exposure: Vec<CountryExposure>,
}

/// ETF sector exposure data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorExposure {
    /// Sector name.
    pub sector: String,
    /// Exposure percentage.
    pub exposure: f64,
}

/// ETF sector exposure response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETFSectorExposure {
    /// ETF symbol.
    pub symbol: String,
    /// Array of sector exposures.
    #[serde(rename = "sectorExposure")]
    pub sector_exposure: Vec<SectorExposure>,
}