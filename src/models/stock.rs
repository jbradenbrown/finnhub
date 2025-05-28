//! Stock-related data models.

use super::common::Currency;
use serde::{Deserialize, Serialize};

/// Stock quote data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Current price.
    #[serde(rename = "c")]
    pub current_price: f64,
    /// Change.
    #[serde(rename = "d")]
    pub change: f64,
    /// Percent change.
    #[serde(rename = "dp")]
    pub percent_change: f64,
    /// High price of the day.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price of the day.
    #[serde(rename = "l")]
    pub low: f64,
    /// Open price of the day.
    #[serde(rename = "o")]
    pub open: f64,
    /// Previous close price.
    #[serde(rename = "pc")]
    pub previous_close: f64,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: i64,
}

/// Company profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
    /// Country of company's headquarters.
    pub country: String,
    /// Currency used in company filings.
    pub currency: Currency,
    /// Listed exchange.
    pub exchange: String,
    /// Finnhub industry classification.
    pub finnhub_industry: String,
    /// IPO date.
    pub ipo: Option<String>,
    /// Company logo URL.
    pub logo: Option<String>,
    /// Market capitalization.
    pub market_capitalization: f64,
    /// Company name.
    pub name: String,
    /// Company phone number.
    pub phone: Option<String>,
    /// Number of employees.
    pub share_outstanding: f64,
    /// Company symbol/ticker.
    pub ticker: String,
    /// Company website.
    pub weburl: Option<String>,
}

/// Stock symbol information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Symbol description.
    pub description: String,
    /// Display symbol.
    pub display_symbol: String,
    /// Symbol/ticker.
    pub symbol: String,
    /// Security type.
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
    /// Currency.
    pub currency: Option<Currency>,
    /// FIGI identifier.
    pub figi: Option<String>,
    /// ISIN.
    pub isin: Option<String>,
    /// MIC code.
    pub mic: Option<String>,
}

/// Stock candles (OHLCV) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockCandles {
    /// List of open prices.
    #[serde(rename = "o")]
    pub open: Vec<f64>,
    /// List of high prices.
    #[serde(rename = "h")]
    pub high: Vec<f64>,
    /// List of low prices.
    #[serde(rename = "l")]
    pub low: Vec<f64>,
    /// List of close prices.
    #[serde(rename = "c")]
    pub close: Vec<f64>,
    /// List of volume data.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,
    /// List of timestamps.
    #[serde(rename = "t")]
    pub timestamp: Vec<i64>,
    /// Status of the response.
    #[serde(rename = "s")]
    pub status: String,
}

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

impl std::fmt::Display for CandleResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
