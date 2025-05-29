//! Miscellaneous data models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI chat message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AIChatMessage {
    /// Role (system/user).
    pub role: String,
    /// Content.
    pub content: String,
}

/// AI chat request.
#[derive(Debug, Serialize, Deserialize)]
pub struct AIChatRequest {
    /// Messages.
    pub messages: Vec<AIChatMessage>,
    /// Stream responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// AI chat response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIChatResponse {
    /// Chat ID.
    pub chat_id: String,
    /// Response text.
    pub content: String,
    /// Query summary.
    pub query_summary: String,
    /// Related queries.
    pub related_queries: Vec<String>,
    /// List of tickers mentioned.
    pub tickers: Vec<serde_json::Value>,
    /// Sources.
    pub sources: Vec<serde_json::Value>,
    /// Widgets.
    pub widgets: Vec<String>,
}

/// Airline price index data point.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirlinePriceIndex {
    /// Airline name.
    pub airline: String,
    /// Date.
    pub date: String,
    /// Price index.
    pub price_index: f64,
    /// Daily average ticket price.
    pub daily_avg_price: f64,
}

/// Airline price index response.
#[derive(Debug, Deserialize)]
pub struct AirlinePriceIndexData {
    /// Array of price index data.
    pub data: Vec<AirlinePriceIndex>,
    /// Airline name.
    pub airline: String,
    /// From date.
    pub from: String,
    /// To date.
    pub to: String,
}

/// Country metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryMetadata {
    /// Country name.
    pub country: String,
    /// Alpha 2 code.
    pub code2: String,
    /// Alpha 3 code.
    pub code3: String,
    /// UN code.
    pub code_no: String,
    /// Currency name.
    pub currency: String,
    /// Currency code.
    pub currency_code: String,
    /// Region.
    pub region: String,
    /// Sub-region.
    pub sub_region: String,
    /// Moody's credit risk rating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<String>,
    /// Default spread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_spread: Option<f64>,
    /// Country risk premium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_risk_premium: Option<f64>,
    /// Equity risk premium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_risk_premium: Option<f64>,
}

/// COVID-19 information.
#[derive(Debug, Deserialize)]
pub struct CovidInfo {
    /// State.
    pub state: String,
    /// Number of confirmed cases.
    #[serde(rename = "case")]
    pub cases: f64,
    /// Number of confirmed deaths.
    pub death: f64,
    /// Updated time.
    pub updated: String,
}

/// FDA committee meeting.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FDACommitteeMeeting {
    /// Start time of the event in EST.
    pub from_date: String,
    /// End time of the event in EST.
    pub to_date: String,
    /// Event's description.
    pub event_description: String,
    /// URL.
    pub url: String,
}

/// Technical indicator response.
#[derive(Debug, Deserialize)]
pub struct TechnicalIndicator {
    /// Indicator values (key is indicator name).
    #[serde(flatten)]
    pub indicators: HashMap<String, Vec<f64>>,
    /// Timestamps.
    #[serde(rename = "t")]
    pub timestamps: Vec<i64>,
}

/// Major development.
#[derive(Debug, Deserialize)]
pub struct Development {
    /// Company symbol.
    pub symbol: String,
    /// Published time.
    pub datetime: String,
    /// Development headline.
    pub headline: String,
    /// Development description.
    pub description: String,
    /// URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Press release response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressRelease {
    /// Company symbol.
    pub symbol: String,
    /// Array of major developments.
    pub major_development: Vec<Development>,
}

/// Symbol lookup info.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLookupInfo {
    /// Symbol description.
    pub description: String,
    /// Display symbol name.
    pub display_symbol: String,
    /// Unique symbol.
    pub symbol: String,
    /// Security type.
    #[serde(rename = "type")]
    pub security_type: String,
}

/// Symbol lookup response.
#[derive(Debug, Deserialize)]
pub struct SymbolLookup {
    /// Number of results.
    pub count: i64,
    /// Array of search results.
    pub result: Vec<SymbolLookupInfo>,
}

/// Sector metric data.
#[derive(Debug, Deserialize)]
pub struct SectorMetricData {
    /// Sector name.
    pub sector: String,
    /// Metrics data in key-value format.
    pub metrics: HashMap<String, serde_json::Value>,
}

/// Sector metric response.
#[derive(Debug, Deserialize)]
pub struct SectorMetric {
    /// Region.
    pub region: String,
    /// Metrics for each sector.
    pub data: Vec<SectorMetricData>,
}