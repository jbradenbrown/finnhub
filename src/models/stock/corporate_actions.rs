//! Corporate actions and filings models.

use serde::{Deserialize, Serialize};

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

/// SEC filing data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filing {
    /// Access number.
    #[serde(rename = "accessNumber")]
    pub access_number: Option<String>,
    /// Symbol.
    pub symbol: Option<String>,
    /// CIK.
    pub cik: Option<String>,
    /// Form type.
    pub form: Option<String>,
    /// Filed date.
    #[serde(rename = "filedDate")]
    pub filed_date: Option<String>,
    /// Accepted date.
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<String>,
    /// Report's URL.
    #[serde(rename = "reportUrl")]
    pub report_url: Option<String>,
    /// Filing's URL.
    #[serde(rename = "filingUrl")]
    pub filing_url: Option<String>,
}
