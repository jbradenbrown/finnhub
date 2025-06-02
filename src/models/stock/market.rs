//! Market-related models.

use serde::{Deserialize, Serialize};

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

/// Market status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatus {
    /// Exchange name.
    pub exchange: String,
    /// Market holiday.
    pub holiday: Option<String>,
    /// Whether the market is open.
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    /// Market session.
    pub session: Option<String>,
    /// Market state.
    pub state: Option<String>,
    /// Market timezone.
    pub timezone: String,
    /// Current timestamp.
    #[serde(rename = "t")]
    pub timestamp: i64,
}

/// Investment theme portfolio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentTheme {
    /// Theme name.
    pub theme: String,
    /// Array of stocks in the theme.
    pub data: Vec<ThemeStock>,
}

/// Stock in an investment theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeStock {
    /// Stock symbol.
    pub symbol: String,
}

