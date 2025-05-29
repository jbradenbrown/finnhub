//! Common enums and types used across stock models.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Candle resolution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CandleResolution {
    /// 1 minute
    #[serde(rename = "1")]
    OneMinute,
    /// 5 minutes
    #[serde(rename = "5")]
    FiveMinutes,
    /// 15 minutes
    #[serde(rename = "15")]
    FifteenMinutes,
    /// 30 minutes
    #[serde(rename = "30")]
    ThirtyMinutes,
    /// 60 minutes
    #[serde(rename = "60")]
    SixtyMinutes,
    /// Daily
    #[serde(rename = "D")]
    Daily,
    /// Weekly
    #[serde(rename = "W")]
    Weekly,
    /// Monthly
    #[serde(rename = "M")]
    Monthly,
}

impl fmt::Display for CandleResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandleResolution::OneMinute => write!(f, "1"),
            CandleResolution::FiveMinutes => write!(f, "5"),
            CandleResolution::FifteenMinutes => write!(f, "15"),
            CandleResolution::ThirtyMinutes => write!(f, "30"),
            CandleResolution::SixtyMinutes => write!(f, "60"),
            CandleResolution::Daily => write!(f, "D"),
            CandleResolution::Weekly => write!(f, "W"),
            CandleResolution::Monthly => write!(f, "M"),
        }
    }
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