//! Historical data models.

use serde::{Deserialize, Serialize};

/// Market cap data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketCapData {
    /// Date.
    #[serde(rename = "atDate")]
    pub at_date: String,
    /// Market capitalization.
    #[serde(rename = "marketCapitalization")]
    pub market_capitalization: f64,
}

/// Historical market cap data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMarketCapData {
    /// Symbol.
    pub symbol: String,
    /// Currency.
    pub currency: String,
    /// Market cap data.
    pub data: Vec<MarketCapData>,
}

/// Employee count data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCountData {
    /// Date.
    #[serde(rename = "atDate")]
    pub at_date: String,
    /// Employee count.
    #[serde(rename = "employeeTotal")]
    pub employee_total: i64,
}

/// Historical employee count data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEmployeeCount {
    /// Symbol.
    pub symbol: String,
    /// Employee count data.
    pub data: Vec<EmployeeCountData>,
}

/// ESG score data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGData {
    /// Date.
    #[serde(rename = "atDate")]
    pub at_date: String,
    /// Environmental score.
    #[serde(rename = "environmentScore")]
    pub environment_score: Option<f64>,
    /// Governance score.
    #[serde(rename = "governanceScore")]
    pub governance_score: Option<f64>,
    /// Social score.
    #[serde(rename = "socialScore")]
    pub social_score: Option<f64>,
    /// Total ESG score.
    #[serde(rename = "totalScore")]
    pub total_score: Option<f64>,
}

/// Historical ESG data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalESG {
    /// Symbol.
    pub symbol: String,
    /// ESG data.
    pub data: Vec<ESGData>,
}
