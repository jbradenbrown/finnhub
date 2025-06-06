//! Ownership data models.

use serde::{Deserialize, Serialize};

/// Ownership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ownership {
    /// Name.
    pub name: String,
    /// Share.
    pub share: i64,
    /// Change.
    pub change: Option<i64>,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: String,
}

/// Ownership data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipData {
    /// Symbol.
    pub symbol: String,
    /// Ownership data.
    pub ownership: Vec<Ownership>,
}

/// Fund ownership data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundOwnership {
    /// Symbol.
    pub symbol: String,
    /// Array of fund owners.
    pub ownership: Vec<FundOwner>,
}

/// Fund owner information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundOwner {
    /// Name of the fund.
    pub name: String,
    /// Number of shares held.
    pub share: i64,
    /// Change in shares from previous period.
    pub change: i64,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: String,
    /// Percentage of the fund's portfolio.
    #[serde(rename = "portfolioPercent")]
    pub portfolio_percent: Option<f64>,
}