//! Scanner/Technical Analysis models.

use serde::Deserialize;
use std::collections::HashMap;

/// Pattern data point.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanPattern {
    /// Pattern name.
    pub patternname: String,
    /// Pattern type (bullish/bearish).
    pub patterntype: String,
    /// Symbol.
    pub symbol: String,
    /// Pattern status.
    pub status: String,
    /// A price.
    pub aprice: f64,
    /// A time.
    pub atime: i64,
    /// B price.
    pub bprice: f64,
    /// B time.
    pub btime: i64,
    /// C price.
    pub cprice: f64,
    /// C time.
    pub ctime: i64,
    /// D price.
    pub dprice: f64,
    /// D time.
    pub dtime: i64,
    /// Entry price.
    pub entry: f64,
    /// Stop loss.
    pub stoploss: f64,
    /// First profit target.
    pub profit1: f64,
    /// Second profit target.
    pub profit2: f64,
    /// Sort time.
    pub sort_time: i64,
    /// Pattern additional fields.
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Pattern recognition response.
#[derive(Debug, Deserialize)]
pub struct PatternRecognition {
    /// Array of patterns.
    pub points: Vec<ScanPattern>,
}

/// Support and resistance levels.
#[derive(Debug, Deserialize)]
pub struct SupportResistance {
    /// Array of support and resistance levels.
    pub levels: Vec<f64>,
}

/// Indicator count.
#[derive(Debug, Deserialize)]
pub struct IndicatorCount {
    /// Number of buy signals.
    pub buy: i64,
    /// Number of neutral signals.
    pub neutral: i64,
    /// Number of sell signals.
    pub sell: i64,
}

/// Technical analysis summary.
#[derive(Debug, Deserialize)]
pub struct TechnicalAnalysis {
    /// Number of indicators for each signal.
    pub count: IndicatorCount,
    /// Aggregate signal.
    pub signal: String,
}

/// Trend information.
#[derive(Debug, Deserialize)]
pub struct Trend {
    /// ADX reading.
    pub adx: f64,
    /// Whether market is trending or going sideways.
    pub trending: bool,
}

/// Aggregate indicators response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateIndicators {
    /// Technical analysis signals.
    pub technical_analysis: TechnicalAnalysis,
    /// Trend information.
    pub trend: Trend,
}