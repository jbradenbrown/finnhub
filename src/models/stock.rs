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

/// Basic financials data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicFinancials {
    /// Company symbol.
    pub symbol: String,
    /// Metric type.
    pub metric_type: String,
    /// Map of key metrics.
    pub metric: serde_json::Value,
    /// Map of time-series data.
    pub series: Option<serde_json::Value>,
}

/// Earnings data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earnings {
    /// Actual earnings.
    pub actual: Option<f64>,
    /// Estimated earnings.
    pub estimate: Option<f64>,
    /// Earnings period.
    pub period: String,
    /// Earnings surprise.
    pub surprise: Option<f64>,
    /// Surprise percentage.
    #[serde(rename = "surprisePercent")]
    pub surprise_percent: Option<f64>,
    /// Company symbol.
    pub symbol: String,
}

/// Dividend data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dividend {
    /// Company symbol.
    pub symbol: String,
    /// Ex-dividend date.
    pub ex_dividend_date: String,
    /// Payment date.
    pub payment_date: String,
    /// Record date.
    pub record_date: String,
    /// Declaration date.
    pub declaration_date: String,
    /// Dividend amount.
    pub amount: f64,
    /// Adjusted dividend amount.
    pub adjusted_amount: f64,
    /// Currency.
    pub currency: String,
}

/// Stock split data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSplit {
    /// Company symbol.
    pub symbol: String,
    /// Split date.
    pub date: String,
    /// From factor.
    #[serde(rename = "fromFactor")]
    pub from_factor: f64,
    /// To factor.
    #[serde(rename = "toFactor")]
    pub to_factor: f64,
}

/// Market cap data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCapData {
    /// Date of the reading.
    pub at_date: String,
    /// Market capitalization value.
    pub market_capitalization: f64,
}

/// Historical market cap data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMarketCapData {
    /// Company symbol.
    pub symbol: String,
    /// Currency.
    pub currency: String,
    /// Array of market cap data.
    pub data: Vec<MarketCapData>,
}

/// Employee count data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeCountData {
    /// Date of the reading.
    pub at_date: String,
    /// Employee count.
    pub employee_count: i64,
}

/// Historical employee count data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEmployeeCount {
    /// Company symbol.
    pub symbol: String,
    /// Array of employee count data.
    pub data: Vec<EmployeeCountData>,
}

/// ESG score data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ESGData {
    /// Date of the reading.
    pub at_date: String,
    /// Environmental score.
    pub environment_score: Option<f64>,
    /// Social score.
    pub social_score: Option<f64>,
    /// Governance score.
    pub governance_score: Option<f64>,
    /// Total ESG score.
    pub total_score: Option<f64>,
}

/// Historical ESG data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalESG {
    /// Company symbol.
    pub symbol: String,
    /// Array of ESG data.
    pub data: Vec<ESGData>,
}

/// Market status data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStatus {
    /// Exchange code.
    pub exchange: String,
    /// Timezone.
    pub timezone: String,
    /// Market session (pre-market, regular, post-market, or null).
    pub session: Option<String>,
    /// Holiday event.
    pub holiday: Option<String>,
    /// Whether the market is open.
    pub is_open: bool,
    /// Current timestamp.
    pub t: i64,
}

/// Ownership data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ownership {
    /// Owner name.
    pub name: String,
    /// Number of shares owned.
    pub share: i64,
    /// Share change.
    pub change: Option<i64>,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: Option<String>,
}

/// Ownership response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipData {
    /// Company symbol.
    pub symbol: String,
    /// Array of ownership data.
    pub ownership: Vec<Ownership>,
}

/// Revenue breakdown data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueBreakdown {
    /// Company symbol.
    pub symbol: String,
    /// Revenue breakdown by category.
    pub data: Vec<serde_json::Value>,
}

/// Insider sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderSentiment {
    /// Company symbol.
    pub symbol: String,
    /// Year.
    pub year: i32,
    /// Month.
    pub month: i32,
    /// Change.
    pub change: i64,
    /// MSPR (Monthly Share Purchase Ratio).
    pub mspr: f64,
}

/// Insider sentiment response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderSentimentData {
    /// Company symbol.
    pub symbol: String,
    /// Array of insider sentiment data.
    pub data: Vec<InsiderSentiment>,
}

/// Upgrade/downgrade data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeDowngrade {
    /// Company symbol.
    pub symbol: String,
    /// Upgrade/downgrade time.
    pub grade_time: i64,
    /// From grade.
    pub from_grade: Option<String>,
    /// To grade.
    pub to_grade: Option<String>,
    /// Company/analyst who did the upgrade/downgrade.
    pub company: String,
    /// Action (up, down, main, init, reit).
    pub action: String,
}

/// Social sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSentiment {
    /// Company symbol.
    pub symbol: String,
    /// Data.
    pub data: Vec<SocialSentimentData>,
}

/// Social sentiment data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSentimentData {
    /// Date.
    pub at_time: String,
    /// Mention count.
    pub mention: i64,
    /// Positive mention count.
    pub positive_mention: i64,
    /// Negative mention count.
    pub negative_mention: i64,
    /// Positive score.
    pub positive_score: f64,
    /// Negative score.
    pub negative_score: f64,
    /// Overall score.
    pub score: f64,
}

/// Supply chain relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainRelationship {
    /// Symbol.
    pub symbol: String,
    /// Company name.
    pub name: String,
    /// Country.
    pub country: Option<String>,
    /// 1-tier supplier.
    #[serde(rename = "oneMonthCorrelation")]
    pub one_month_correlation: Option<f64>,
    /// 1-year correlation.
    #[serde(rename = "oneYearCorrelation")]
    pub one_year_correlation: Option<f64>,
    /// 6-month correlation.
    #[serde(rename = "sixMonthCorrelation")]
    pub six_month_correlation: Option<f64>,
    /// 3-month correlation.
    #[serde(rename = "threeMonthCorrelation")]
    pub three_month_correlation: Option<f64>,
    /// 2-week correlation.
    #[serde(rename = "twoWeekCorrelation")]
    pub two_week_correlation: Option<f64>,
    /// 2-year correlation.
    #[serde(rename = "twoYearCorrelation")]
    pub two_year_correlation: Option<f64>,
}

/// Supply chain data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainData {
    /// Company symbol.
    pub symbol: String,
    /// List of suppliers.
    pub data: Vec<SupplyChainRelationship>,
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
