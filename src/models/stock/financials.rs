//! Financial data models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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

/// Financial statement type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatementType {
    /// Balance sheet
    #[serde(rename = "bs")]
    BalanceSheet,
    /// Income statement
    #[serde(rename = "ic")]
    IncomeStatement,
    /// Cash flow statement
    #[serde(rename = "cf")]
    CashFlow,
}

impl fmt::Display for StatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatementType::BalanceSheet => write!(f, "bs"),
            StatementType::IncomeStatement => write!(f, "ic"),
            StatementType::CashFlow => write!(f, "cf"),
        }
    }
}

/// Financial statement frequency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatementFrequency {
    /// Annual
    #[serde(rename = "annual")]
    Annual,
    /// Quarterly
    #[serde(rename = "quarterly")]
    Quarterly,
    /// TTM (Trailing Twelve Months)
    #[serde(rename = "ttm")]
    TTM,
}

impl fmt::Display for StatementFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatementFrequency::Annual => write!(f, "annual"),
            StatementFrequency::Quarterly => write!(f, "quarterly"),
            StatementFrequency::TTM => write!(f, "ttm"),
        }
    }
}


