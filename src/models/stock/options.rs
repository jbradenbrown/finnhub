//! Option chain data models.

use serde::{Deserialize, Serialize};

/// A single option contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    /// OCC contract symbol (e.g. `AAPL260424C00110000`).
    pub contract_name: String,
    /// Contract size (e.g. `REGULAR`).
    pub contract_size: Option<String>,
    /// Contract period (e.g. `WEEKLY`, `MONTHLY`).
    pub contract_period: Option<String>,
    /// Currency.
    pub currency: Option<String>,
    /// Option type: `CALL` or `PUT`.
    #[serde(rename = "type")]
    pub option_type: String,
    /// In-the-money flag (`TRUE`/`FALSE`).
    pub in_the_money: Option<String>,
    /// Last trade date-time.
    pub last_trade_date_time: Option<String>,
    /// Expiration date.
    pub expiration_date: String,
    /// Strike price.
    pub strike: f64,
    /// Last traded price.
    pub last_price: Option<f64>,
    /// Bid price.
    pub bid: Option<f64>,
    /// Ask price.
    pub ask: Option<f64>,
    /// Absolute price change.
    pub change: Option<f64>,
    /// Percent price change.
    pub change_percent: Option<f64>,
    /// Trading volume.
    pub volume: Option<i64>,
    /// Open interest.
    pub open_interest: Option<i64>,
    /// Implied volatility.
    pub implied_volatility: Option<f64>,
    /// Delta.
    pub delta: Option<f64>,
    /// Gamma.
    pub gamma: Option<f64>,
    /// Theta.
    pub theta: Option<f64>,
    /// Vega.
    pub vega: Option<f64>,
    /// Rho.
    pub rho: Option<f64>,
    /// Theoretical price.
    pub theoretical: Option<f64>,
    /// Intrinsic value.
    pub intrinsic_value: Option<f64>,
    /// Time value.
    pub time_value: Option<f64>,
    /// Last data update time.
    pub updated_at: Option<String>,
    /// Days remaining until expiration.
    pub days_before_expiration: Option<i64>,
}

/// Calls and puts grouped by side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContracts {
    /// Call contracts.
    #[serde(rename = "CALL", default)]
    pub call: Vec<OptionContract>,
    /// Put contracts.
    #[serde(rename = "PUT", default)]
    pub put: Vec<OptionContract>,
}

/// Option chain for a single expiration date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainExpiration {
    /// Expiration date.
    pub expiration_date: String,
    /// Implied volatility for the expiration.
    pub implied_volatility: Option<f64>,
    /// Total put volume.
    pub put_volume: Option<i64>,
    /// Total call volume.
    pub call_volume: Option<i64>,
    /// Put/call volume ratio.
    pub put_call_volume_ratio: Option<f64>,
    /// Total put open interest.
    pub put_open_interest: Option<i64>,
    /// Total call open interest.
    pub call_open_interest: Option<i64>,
    /// Put/call open interest ratio.
    pub put_call_open_interest_ratio: Option<f64>,
    /// Number of option contracts in this expiration.
    pub options_count: Option<i64>,
    /// Calls and puts.
    pub options: OptionContracts,
}

/// Option chain response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    /// Symbol code.
    pub code: String,
    /// Exchange.
    pub exchange: Option<String>,
    /// Last trade date for the underlying.
    pub last_trade_date: Option<String>,
    /// Last trade price for the underlying.
    pub last_trade_price: Option<f64>,
    /// One entry per expiration date.
    #[serde(default)]
    pub data: Vec<OptionChainExpiration>,
}
