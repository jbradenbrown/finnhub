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

/// Earnings quality score indicators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsQualityScore {
    /// Symbol.
    pub symbol: String,
    /// Frequency.
    pub freq: String,
    /// Period.
    pub period: String,
    /// Non-recurring items to revenue ratio.
    #[serde(rename = "nonRecurringItemsToRevenue")]
    pub non_recurring_items_to_revenue: Option<f64>,
    /// Non-recurring items to revenue TTM.
    #[serde(rename = "nonRecurringItemsToRevenueTTM")]
    pub non_recurring_items_to_revenue_ttm: Option<f64>,
    /// Non-recurring items total.
    #[serde(rename = "nonRecurringItemsTotal")]
    pub non_recurring_items_total: Option<f64>,
    /// Non-recurring items total TTM.
    #[serde(rename = "nonRecurringItemsTotalTTM")]
    pub non_recurring_items_total_ttm: Option<f64>,
    /// Letter score.
    #[serde(rename = "letterScore")]
    pub letter_score: Option<String>,
    /// Score.
    pub score: Option<f64>,
}