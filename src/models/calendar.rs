//! Calendar-related data models.

use serde::{Deserialize, Serialize};

/// Earnings release data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsRelease {
    /// Symbol.
    pub symbol: Option<String>,
    /// Date.
    pub date: Option<String>,
    /// Indicates whether the earnings is announced before market open(bmo), after market close(amc), or during market hour(dmh).
    pub hour: Option<String>,
    /// Earnings year.
    pub year: Option<i64>,
    /// Earnings quarter.
    pub quarter: Option<i64>,
    /// EPS estimate.
    #[serde(rename = "epsEstimate")]
    pub eps_estimate: Option<f64>,
    /// EPS actual.
    #[serde(rename = "epsActual")]
    pub eps_actual: Option<f64>,
    /// Revenue estimate.
    #[serde(rename = "revenueEstimate")]
    pub revenue_estimate: Option<f64>,
    /// Revenue actual.
    #[serde(rename = "revenueActual")]
    pub revenue_actual: Option<f64>,
}

/// Earnings calendar response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCalendar {
    /// Array of earnings releases.
    #[serde(rename = "earningsCalendar")]
    pub earnings_calendar: Vec<EarningsRelease>,
}

/// Economic event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    /// Actual release.
    pub actual: Option<f64>,
    /// Previous release.
    pub prev: Option<f64>,
    /// Country.
    pub country: Option<String>,
    /// Unit.
    pub unit: Option<String>,
    /// Estimate.
    pub estimate: Option<f64>,
    /// Event name.
    pub event: Option<String>,
    /// Impact level.
    pub impact: Option<String>,
    /// Release time.
    pub time: Option<String>,
}

/// Economic calendar response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicCalendar {
    /// Array of economic events.
    #[serde(rename = "economicCalendar")]
    pub economic_calendar: Vec<EconomicEvent>,
}

/// IPO event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPOEvent {
    /// Symbol.
    pub symbol: Option<String>,
    /// IPO date.
    pub date: Option<String>,
    /// Exchange.
    pub exchange: Option<String>,
    /// Company's name.
    pub name: Option<String>,
    /// IPO status. Can take 1 of the following values: expected, priced, withdrawn, filed.
    pub status: Option<String>,
    /// Projected price or price range.
    pub price: Option<String>,
    /// Number of shares offered during the IPO.
    #[serde(rename = "numberOfShares")]
    pub number_of_shares: Option<f64>,
    /// Total shares value.
    #[serde(rename = "totalSharesValue")]
    pub total_shares_value: Option<f64>,
}

/// IPO calendar data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPOCalendar {
    /// Array of IPO events.
    #[serde(rename = "ipoCalendar")]
    pub ipo_calendar: Vec<IPOEvent>,
}
