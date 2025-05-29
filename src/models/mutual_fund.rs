//! Mutual fund-related data models.

use serde::{Deserialize, Serialize};

/// Mutual fund profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundProfile {
    /// Name.
    pub name: Option<String>,
    /// Fund's category.
    pub category: Option<String>,
    /// Investment segment.
    #[serde(rename = "investmentSegment")]
    pub investment_segment: Option<String>,
    /// Total NAV.
    #[serde(rename = "totalNav")]
    pub total_nav: Option<f64>,
    /// Expense ratio.
    #[serde(rename = "expenseRatio")]
    pub expense_ratio: Option<f64>,
    /// Index benchmark.
    pub benchmark: Option<String>,
    /// Inception date.
    #[serde(rename = "inceptionDate")]
    pub inception_date: Option<String>,
    /// Fund's description.
    pub description: Option<String>,
    /// Fund family.
    #[serde(rename = "fundFamily")]
    pub fund_family: Option<String>,
    /// Fund company.
    #[serde(rename = "fundCompany")]
    pub fund_company: Option<String>,
    /// Fund's managers.
    pub manager: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Beta.
    pub beta: Option<f64>,
    /// Deferred load.
    #[serde(rename = "deferredLoad")]
    pub deferred_load: Option<f64>,
    /// 12B-1 fee.
    #[serde(rename = "fee12b1")]
    pub fee_12b1: Option<f64>,
    /// Front load.
    #[serde(rename = "frontLoad")]
    pub front_load: Option<f64>,
    /// IRA minimum investment.
    #[serde(rename = "iraMinInvestment")]
    pub ira_min_investment: Option<f64>,
    /// ISIN.
    pub isin: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// Max redemption fee.
    #[serde(rename = "maxRedemptionFee")]
    pub max_redemption_fee: Option<f64>,
    /// Standard minimum investment.
    #[serde(rename = "standardMinInvestment")]
    pub standard_min_investment: Option<f64>,
    /// Turnover.
    pub turnover: Option<f64>,
    /// Fund's series ID.
    #[serde(rename = "seriesId")]
    pub series_id: Option<String>,
    /// Fund's series name.
    #[serde(rename = "seriesName")]
    pub series_name: Option<String>,
    /// Class ID.
    #[serde(rename = "classId")]
    pub class_id: Option<String>,
    /// Class name.
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    /// SFDR classification for EU funds.
    #[serde(rename = "sfdrClassification")]
    pub sfdr_classification: Option<String>,
    /// Fund's currency.
    pub currency: Option<String>,
}

/// Mutual fund holding data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundHolding {
    /// Symbol.
    pub symbol: Option<String>,
    /// Security name.
    pub name: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// Number of shares.
    pub share: Option<f64>,
    /// Portfolio's percent.
    pub percent: Option<f64>,
    /// Market value.
    pub value: Option<f64>,
    /// Asset type (Equity, ETP, Fund, Bond, Other).
    #[serde(rename = "assetType")]
    pub asset_type: Option<String>,
}

/// Mutual fund holdings response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundHoldings {
    /// Symbol.
    pub symbol: String,
    /// As of date.
    #[serde(rename = "atDate")]
    pub at_date: Option<String>,
    /// Total number of holdings.
    #[serde(rename = "numberOfHoldings")]
    pub number_of_holdings: Option<i64>,
    /// Array of holdings.
    pub holdings: Vec<MutualFundHolding>,
}

/// Country exposure data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundCountryExposure {
    /// Country name.
    pub country: String,
    /// Exposure percentage.
    pub exposure: f64,
}

/// Mutual fund country exposure response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundCountryExposureData {
    /// Symbol.
    pub symbol: String,
    /// Array of country exposures.
    #[serde(rename = "countryExposure")]
    pub country_exposure: Vec<MutualFundCountryExposure>,
}

/// Sector exposure data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundSectorExposure {
    /// Sector name.
    pub sector: String,
    /// Exposure percentage.
    pub exposure: f64,
}

/// Mutual fund sector exposure response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundSectorExposureData {
    /// Symbol.
    pub symbol: String,
    /// Array of sector exposures.
    #[serde(rename = "sectorExposure")]
    pub sector_exposure: Vec<MutualFundSectorExposure>,
}

/// EET (European ESG Template) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundEET {
    /// ISIN.
    pub isin: String,
    /// European ESG Template data as JSON.
    pub data: serde_json::Value,
}

/// EET PAI (Principal Adverse Impact) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundEETPAI {
    /// ISIN.
    pub isin: String,
    /// Principal Adverse Impact data as JSON.
    pub data: serde_json::Value,
}