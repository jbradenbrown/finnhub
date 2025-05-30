//! Financial data models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Financial statements response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatements {
    /// Symbol.
    pub symbol: String,
    /// Financial data.
    pub financials: Vec<HashMap<String, serde_json::Value>>,
}

/// Basic financials data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFinancials {
    /// Symbol.
    pub symbol: String,
    /// Metric data.
    pub metric: HashMap<String, serde_json::Value>,
    /// Metric type.
    #[serde(rename = "metricType")]
    pub metric_type: String,
    /// Series data.
    pub series: Option<serde_json::Value>,
}

/// Financial report data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialReport {
    /// Access number.
    #[serde(rename = "accessNumber")]
    pub access_number: Option<String>,
    /// Symbol.
    pub symbol: Option<String>,
    /// CIK.
    pub cik: Option<String>,
    /// Year.
    pub year: Option<i64>,
    /// Quarter.
    pub quarter: Option<i64>,
    /// Form type.
    pub form: Option<String>,
    /// Period start date.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    /// Period end date.
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    /// Filed date.
    #[serde(rename = "filedDate")]
    pub filed_date: Option<String>,
    /// Accepted date.
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<String>,
    /// Report data as JSON object.
    pub report: Option<serde_json::Value>,
}

/// Financials as reported.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialsAsReported {
    /// Symbol.
    pub symbol: Option<String>,
    /// CIK.
    pub cik: Option<String>,
    /// Array of financial reports.
    pub data: Vec<FinancialReport>,
}

/// Earnings data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earnings {
    /// Actual earnings.
    pub actual: Option<f64>,
    /// Estimated earnings.
    pub estimate: Option<f64>,
    /// Earnings period.
    pub period: String,
    /// Earnings surprise.
    pub surprise: Option<f64>,
    /// Earnings surprise percentage.
    #[serde(rename = "surprisePercent")]
    pub surprise_percent: Option<f64>,
    /// Symbol.
    pub symbol: String,
}

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

/// Revenue breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueBreakdown {
    /// Symbol.
    pub symbol: String,
    /// CIK.
    pub cik: Option<String>,
    /// Revenue breakdown data.
    pub data: Vec<HashMap<String, serde_json::Value>>,
}