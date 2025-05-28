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

/// Financial statement type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatementType {
    /// Balance sheet
    #[serde(rename = "bs")]
    BalanceSheet,
    /// Income statement
    #[serde(rename = "ic")]
    IncomeStatement,
    /// Cash flow
    #[serde(rename = "cf")]
    CashFlow,
}

impl std::fmt::Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    /// Trailing twelve months (only for IC and CF)
    #[serde(rename = "ttm")]
    TTM,
    /// Year to date (only for CF)
    #[serde(rename = "ytd")]
    YTD,
}

impl std::fmt::Display for StatementFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementFrequency::Annual => write!(f, "annual"),
            StatementFrequency::Quarterly => write!(f, "quarterly"),
            StatementFrequency::TTM => write!(f, "ttm"),
            StatementFrequency::YTD => write!(f, "ytd"),
        }
    }
}

/// Financial statements response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatements {
    /// Company symbol.
    pub symbol: String,
    /// Array of financial data for each period.
    pub financials: Vec<serde_json::Value>,
}

/// Price target data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTarget {
    /// Company symbol.
    pub symbol: String,
    /// Highest analysts' target.
    pub target_high: f64,
    /// Lowest analysts' target.
    pub target_low: f64,
    /// Mean of all analysts' targets.
    pub target_mean: f64,
    /// Median of all analysts' targets.
    pub target_median: f64,
    /// Number of analysts.
    pub number_analysts: i64,
    /// Last updated time.
    pub last_updated: String,
}

/// Recommendation trend data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationTrend {
    /// Company symbol.
    pub symbol: String,
    /// Number of buy recommendations.
    pub buy: i64,
    /// Number of hold recommendations.
    pub hold: i64,
    /// Updated period.
    pub period: String,
    /// Number of sell recommendations.
    pub sell: i64,
    /// Number of strong buy recommendations.
    pub strong_buy: i64,
    /// Number of strong sell recommendations.
    pub strong_sell: i64,
}

/// Insider transactions data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTransaction {
    /// Company symbol.
    pub symbol: String,
    /// Insider name.
    pub name: String,
    /// Number of shares traded.
    pub share: i64,
    /// Share change.
    pub change: i64,
    /// Transaction date.
    pub transaction_date: String,
    /// Transaction price.
    pub transaction_price: f64,
    /// Transaction code.
    pub transaction_code: String,
    /// Filing date.
    pub filing_date: String,
}

/// Insider transactions response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransactions {
    /// Company symbol.
    pub symbol: String,
    /// Array of insider transactions.
    pub data: Vec<InsiderTransaction>,
}
