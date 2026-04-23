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

/// Institutional investor profile entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalProfileInfo {
    /// CIK.
    pub cik: String,
    /// Firm type.
    pub firm_type: Option<String>,
    /// Manager.
    pub manager: Option<String>,
    /// Investment philosophy.
    pub philosophy: Option<String>,
    /// Profile description.
    pub profile: Option<String>,
    /// Profile image URL.
    pub profile_img: Option<String>,
}

/// Institutional profile response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalProfile {
    /// CIK filter (empty when listing all).
    #[serde(default)]
    pub cik: String,
    /// Array of institutional profiles.
    pub data: Vec<InstitutionalProfileInfo>,
}

/// Single position held by an institutional investor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalPortfolioPosition {
    /// Change in shares from previous period.
    pub change: Option<i64>,
    /// CUSIP of the holding.
    #[serde(default)]
    pub cusip: Option<String>,
    /// Holding name.
    pub name: Option<String>,
    /// Shares with no voting authority.
    #[serde(rename = "noVoting")]
    pub no_voting: Option<i64>,
    /// Percentage of the portfolio.
    pub percentage: Option<f64>,
    /// Put/call indicator.
    #[serde(rename = "putCall")]
    pub put_call: Option<String>,
    /// Total share count.
    pub share: Option<i64>,
    /// Shares with shared voting authority.
    #[serde(rename = "sharedVoting")]
    pub shared_voting: Option<i64>,
    /// Shares with sole voting authority.
    #[serde(rename = "soleVoting")]
    pub sole_voting: Option<i64>,
    /// Symbol.
    pub symbol: Option<String>,
    /// Position market value.
    pub value: Option<f64>,
}

/// Institutional portfolio snapshot for a filing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalPortfolioGroup {
    /// Filing date.
    pub filing_date: String,
    /// Report date.
    pub report_date: String,
    /// Portfolio positions.
    pub portfolio: Vec<InstitutionalPortfolioPosition>,
}

/// Institutional portfolio response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalPortfolio {
    /// Fund's CIK.
    pub cik: String,
    /// Fund's name.
    pub name: Option<String>,
    /// Portfolio history.
    pub data: Vec<InstitutionalPortfolioGroup>,
}

/// Single institutional investor's position in a stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalOwnershipPosition {
    /// Change in shares from previous period.
    pub change: Option<i64>,
    /// Investor's CIK.
    pub cik: String,
    /// Investor name.
    pub name: Option<String>,
    /// Shares with no voting authority.
    #[serde(rename = "noVoting")]
    pub no_voting: Option<i64>,
    /// Percentage of shares outstanding.
    pub percentage: Option<f64>,
    /// Put/call indicator.
    #[serde(rename = "putCall")]
    pub put_call: Option<String>,
    /// Share count.
    pub share: Option<i64>,
    /// Shares with shared voting authority.
    #[serde(rename = "sharedVoting")]
    pub shared_voting: Option<i64>,
    /// Shares with sole voting authority.
    #[serde(rename = "soleVoting")]
    pub sole_voting: Option<i64>,
    /// Position market value.
    pub value: Option<f64>,
}

/// Snapshot of all institutional ownership for a stock at a report date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalOwnershipGroup {
    /// Report date.
    pub report_date: String,
    /// Ownership positions.
    pub ownership: Vec<InstitutionalOwnershipPosition>,
}

/// Institutional ownership response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalOwnership {
    /// Symbol.
    pub symbol: String,
    /// CUSIP.
    #[serde(default)]
    pub cusip: String,
    /// Ownership history.
    pub data: Vec<InstitutionalOwnershipGroup>,
}
