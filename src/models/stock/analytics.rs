//! Analytics and recommendations models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Price target data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTarget {
    /// Symbol.
    pub symbol: String,
    /// Target high.
    #[serde(rename = "targetHigh")]
    pub target_high: f64,
    /// Target low.
    #[serde(rename = "targetLow")]
    pub target_low: f64,
    /// Target mean.
    #[serde(rename = "targetMean")]
    pub target_mean: f64,
    /// Target median.
    #[serde(rename = "targetMedian")]
    pub target_median: f64,
    /// Last updated date.
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

/// Recommendation trend data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationTrend {
    /// Number of analysts with buy rating.
    pub buy: i32,
    /// Number of analysts with hold rating.
    pub hold: i32,
    /// Period.
    pub period: String,
    /// Number of analysts with sell rating.
    pub sell: i32,
    /// Number of analysts with strong buy rating.
    #[serde(rename = "strongBuy")]
    pub strong_buy: i32,
    /// Number of analysts with strong sell rating.
    #[serde(rename = "strongSell")]
    pub strong_sell: i32,
    /// Symbol.
    pub symbol: String,
}

/// Upgrade/downgrade data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDowngrade {
    /// Symbol.
    pub symbol: String,
    /// Grade time.
    #[serde(rename = "gradeTime")]
    pub grade_time: i64,
    /// From grade.
    #[serde(rename = "fromGrade")]
    pub from_grade: Option<String>,
    /// To grade.
    #[serde(rename = "toGrade")]
    pub to_grade: Option<String>,
    /// Company name.
    pub company: String,
    /// Action.
    pub action: String,
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
