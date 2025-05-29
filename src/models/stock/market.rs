//! Market-related models.

use serde::{Deserialize, Serialize};

/// Historical NBBO (National Best Bid and Offer) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalNBBO {
    /// Symbol.
    pub s: String,
    /// Total number of ticks.
    pub total: i64,
    /// Number of ticks skipped.
    pub skip: i64,
    /// Number of ticks returned.
    pub count: i64,
    /// Array of timestamps.
    pub t: Vec<i64>,
    /// Array of ask prices.
    pub a: Vec<f64>,
    /// Array of ask volumes.
    pub av: Vec<i64>,
    /// Array of ask exchanges.
    pub ax: Vec<String>,
    /// Array of bid prices.
    pub b: Vec<f64>,
    /// Array of bid volumes.
    pub bv: Vec<i64>,
    /// Array of bid exchanges.
    pub bx: Vec<String>,
    /// Array of conditions.
    pub c: Vec<Vec<String>>,
}

/// Market holiday data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHoliday {
    /// Exchange code.
    pub exchange: String,
    /// Array of holidays.
    pub data: Vec<Holiday>,
    /// Timezone.
    pub timezone: String,
}

/// Holiday information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    /// Event name.
    #[serde(rename = "eventName")]
    pub event_name: String,
    /// Date of the holiday.
    #[serde(rename = "atDate")]
    pub at_date: String,
    /// Trading hours (if partially open).
    #[serde(rename = "tradingHour")]
    pub trading_hour: String,
}

/// International filing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalFiling {
    /// Symbol.
    pub symbol: String,
    /// Company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Filed date.
    #[serde(rename = "filedDate")]
    pub filed_date: String,
    /// Filing category.
    pub category: String,
    /// Filing title.
    pub title: String,
    /// Filing description.
    pub description: String,
    /// Filing URL.
    pub url: String,
    /// Language.
    pub language: String,
    /// Country.
    pub country: String,
}