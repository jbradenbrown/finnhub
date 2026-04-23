//! Earnings and revenue estimates models.

use serde::{Deserialize, Serialize};

/// EPS estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EPSEstimate {
    /// Average estimate.
    #[serde(rename = "epsAvg")]
    pub eps_avg: Option<f64>,
    /// High estimate.
    #[serde(rename = "epsHigh")]
    pub eps_high: Option<f64>,
    /// Low estimate.
    #[serde(rename = "epsLow")]
    pub eps_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// EPS estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EPSEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of EPS estimates.
    pub data: Vec<EPSEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Revenue estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEstimate {
    /// Average revenue estimate.
    #[serde(rename = "revenueAvg")]
    pub revenue_avg: Option<f64>,
    /// High revenue estimate.
    #[serde(rename = "revenueHigh")]
    pub revenue_high: Option<f64>,
    /// Low revenue estimate.
    #[serde(rename = "revenueLow")]
    pub revenue_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// Revenue estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of revenue estimates.
    pub data: Vec<RevenueEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// EBITDA estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBITDAEstimate {
    /// Average EBITDA estimate.
    #[serde(rename = "ebitdaAvg")]
    pub ebitda_avg: Option<f64>,
    /// High EBITDA estimate.
    #[serde(rename = "ebitdaHigh")]
    pub ebitda_high: Option<f64>,
    /// Low EBITDA estimate.
    #[serde(rename = "ebitdaLow")]
    pub ebitda_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// EBITDA estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBITDAEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of EBITDA estimates.
    pub data: Vec<EBITDAEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// EBIT estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBITEstimate {
    /// Average EBIT estimate.
    #[serde(rename = "ebitAvg")]
    pub ebit_avg: Option<f64>,
    /// High EBIT estimate.
    #[serde(rename = "ebitHigh")]
    pub ebit_high: Option<f64>,
    /// Low EBIT estimate.
    #[serde(rename = "ebitLow")]
    pub ebit_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// EBIT estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBITEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of EBIT estimates.
    pub data: Vec<EBITEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Net income estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetIncomeEstimate {
    /// Average net income estimate.
    #[serde(rename = "netIncomeAvg")]
    pub net_income_avg: Option<f64>,
    /// High net income estimate.
    #[serde(rename = "netIncomeHigh")]
    pub net_income_high: Option<f64>,
    /// Low net income estimate.
    #[serde(rename = "netIncomeLow")]
    pub net_income_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// Net income estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetIncomeEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of net income estimates.
    pub data: Vec<NetIncomeEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Pretax income estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PretaxIncomeEstimate {
    /// Average pretax income estimate.
    #[serde(rename = "pretaxIncomeAvg")]
    pub pretax_income_avg: Option<f64>,
    /// High pretax income estimate.
    #[serde(rename = "pretaxIncomeHigh")]
    pub pretax_income_high: Option<f64>,
    /// Low pretax income estimate.
    #[serde(rename = "pretaxIncomeLow")]
    pub pretax_income_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// Pretax income estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PretaxIncomeEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of pretax income estimates.
    pub data: Vec<PretaxIncomeEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Gross income estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrossIncomeEstimate {
    /// Average gross income estimate.
    #[serde(rename = "grossIncomeAvg")]
    pub gross_income_avg: Option<f64>,
    /// High gross income estimate.
    #[serde(rename = "grossIncomeHigh")]
    pub gross_income_high: Option<f64>,
    /// Low gross income estimate.
    #[serde(rename = "grossIncomeLow")]
    pub gross_income_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// Gross income estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrossIncomeEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of gross income estimates.
    pub data: Vec<GrossIncomeEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Dividend per share estimate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPSEstimate {
    /// Average DPS estimate.
    #[serde(rename = "dpsAvg")]
    pub dps_avg: Option<f64>,
    /// High DPS estimate.
    #[serde(rename = "dpsHigh")]
    pub dps_high: Option<f64>,
    /// Low DPS estimate.
    #[serde(rename = "dpsLow")]
    pub dps_low: Option<f64>,
    /// Number of analysts.
    #[serde(rename = "numberAnalysts")]
    pub number_analysts: Option<i32>,
    /// Period.
    pub period: String,
    /// Year.
    pub year: Option<i32>,
    /// Quarter.
    pub quarter: Option<i32>,
}

/// DPS (dividend per share) estimates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPSEstimates {
    /// Symbol.
    pub symbol: String,
    /// Array of DPS estimates.
    pub data: Vec<DPSEstimate>,
    /// Frequency.
    pub freq: Option<String>,
}

/// Earnings quality score response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsQualityScore {
    /// Symbol.
    pub symbol: String,
    /// Frequency.
    pub freq: String,
    /// Array of earnings quality scores.
    pub data: Vec<EarningsQualityScoreData>,
}

/// Earnings quality score indicators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsQualityScoreData {
    /// Period.
    pub period: String,
    /// Capital allocation score.
    #[serde(rename = "capitalAllocation")]
    pub capital_allocation: Option<f64>,
    /// Growth score.
    pub growth: Option<f64>,
    /// Letter score.
    #[serde(rename = "letterScore")]
    pub letter_score: Option<String>,
    /// Leverage score.
    pub leverage: Option<f64>,
    /// Profitability score.
    pub profitability: Option<f64>,
    /// Overall score.
    pub score: Option<f64>,
}
