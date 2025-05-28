//! Common data types used across the API.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Represents a timestamp in the API responses.
pub type Timestamp = DateTime<Utc>;

/// Represents a date without time information.
pub type Date = NaiveDate;

/// Common response wrapper for paginated results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    /// The data items.
    pub data: Vec<T>,
    /// Total number of items available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// Market status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    /// Market is open.
    Open,
    /// Market is closed.
    Closed,
    /// Pre-market hours.
    #[serde(rename = "pre-market")]
    PreMarket,
    /// After-hours trading.
    #[serde(rename = "after-hours")]
    AfterHours,
}

/// Time resolution for candle data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resolution {
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

impl Resolution {
    /// Convert to API string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneMinute => "1",
            Self::FiveMinutes => "5",
            Self::FifteenMinutes => "15",
            Self::ThirtyMinutes => "30",
            Self::SixtyMinutes => "60",
            Self::Daily => "D",
            Self::Weekly => "W",
            Self::Monthly => "M",
        }
    }
}

/// Exchange codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exchange(pub String);

impl Exchange {
    /// US exchanges.
    pub const US: &'static str = "US";
    /// NYSE.
    pub const NYSE: &'static str = "NYSE";
    /// NASDAQ.
    pub const NASDAQ: &'static str = "NASDAQ";
    /// London Stock Exchange.
    pub const LSE: &'static str = "LSE";
    /// Tokyo Stock Exchange.
    pub const TSE: &'static str = "TSE";
}

/// Currency codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Currency(pub String);

impl Currency {
    /// US Dollar.
    pub const USD: &'static str = "USD";
    /// Euro.
    pub const EUR: &'static str = "EUR";
    /// British Pound.
    pub const GBP: &'static str = "GBP";
    /// Japanese Yen.
    pub const JPY: &'static str = "JPY";
    /// Canadian Dollar.
    pub const CAD: &'static str = "CAD";
    /// Australian Dollar.
    pub const AUD: &'static str = "AUD";
}

/// Represents a price/volume bar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Open price.
    #[serde(rename = "o")]
    pub open: f64,
    /// High price.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price.
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price.
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume.
    #[serde(rename = "v")]
    pub volume: f64,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: i64,
    /// Status (optional).
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Generic key-value metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric name/key.
    pub key: String,
    /// Metric value.
    pub value: serde_json::Value,
}
