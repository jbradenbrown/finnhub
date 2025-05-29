//! Price and market data models.

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

/// Last bid-ask data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAsk {
    /// Bid price.
    #[serde(rename = "b")]
    pub bid: Option<f64>,
    /// Ask price.
    #[serde(rename = "a")]
    pub ask: Option<f64>,
    /// Bid volume.
    #[serde(rename = "bv")]
    pub bid_volume: Option<f64>,
    /// Ask volume.
    #[serde(rename = "av")]
    pub ask_volume: Option<f64>,
    /// Reference UNIX timestamp in ms.
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Stock candles (OHLCV) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockCandles {
    /// List of close prices.
    #[serde(rename = "c")]
    pub close: Vec<f64>,
    /// List of high prices.
    #[serde(rename = "h")]
    pub high: Vec<f64>,
    /// List of low prices.
    #[serde(rename = "l")]
    pub low: Vec<f64>,
    /// List of open prices.
    #[serde(rename = "o")]
    pub open: Vec<f64>,
    /// Status of the response.
    #[serde(rename = "s")]
    pub status: String,
    /// List of timestamps.
    #[serde(rename = "t")]
    pub timestamp: Vec<i64>,
    /// List of volume data.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,
}

/// Tick data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Number of ticks skipped.
    pub skip: i64,
    /// Number of ticks returned.
    pub count: i64,
    /// Total number of ticks for that date.
    pub total: i64,
    /// List of volume data.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,
    /// List of price data.
    #[serde(rename = "p")]
    pub price: Vec<f64>,
    /// List of timestamp in UNIX ms.
    #[serde(rename = "t")]
    pub timestamp: Vec<i64>,
    /// List of venues/exchanges.
    #[serde(rename = "x")]
    pub exchange: Vec<String>,
    /// List of trade conditions.
    #[serde(rename = "c")]
    pub conditions: Option<Vec<Vec<String>>>,
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

/// Price performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMetrics {
    /// Symbol.
    pub symbol: String,
    /// Period performance.
    pub data: PriceMetricsData,
}

/// Price metrics data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMetricsData {
    /// 1 Day performance.
    #[serde(rename = "1D")]
    pub one_day: Option<PricePerformance>,
    /// 1 Week performance.
    #[serde(rename = "1W")]
    pub one_week: Option<PricePerformance>,
    /// 1 Month performance.
    #[serde(rename = "1M")]
    pub one_month: Option<PricePerformance>,
    /// 3 Month performance.
    #[serde(rename = "3M")]
    pub three_month: Option<PricePerformance>,
    /// 6 Month performance.
    #[serde(rename = "6M")]
    pub six_month: Option<PricePerformance>,
    /// Year to date performance.
    #[serde(rename = "YTD")]
    pub ytd: Option<PricePerformance>,
    /// 1 Year performance.
    #[serde(rename = "1Y")]
    pub one_year: Option<PricePerformance>,
    /// 3 Year performance.
    #[serde(rename = "3Y")]
    pub three_year: Option<PricePerformance>,
    /// 5 Year performance.
    #[serde(rename = "5Y")]
    pub five_year: Option<PricePerformance>,
    /// 10 Year performance.
    #[serde(rename = "10Y")]
    pub ten_year: Option<PricePerformance>,
}

/// Price performance for a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePerformance {
    /// Actual price change.
    pub actual: f64,
    /// Percentage change.
    pub percent: f64,
}

/// Dividends v2 data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendsV2 {
    /// Symbol.
    pub symbol: String,
    /// Array of dividend data.
    pub data: Vec<DividendV2>,
}

/// Dividend v2 information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendV2 {
    /// Ex-dividend date.
    #[serde(rename = "exDate")]
    pub ex_date: String,
    /// Dividend amount.
    pub amount: f64,
}