//! Insider activity models.

use serde::{Deserialize, Serialize};

/// Insider transactions data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransaction {
    /// Insider name.
    pub name: String,
    /// Share number.
    pub share: Option<i64>,
    /// Change.
    pub change: Option<i64>,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: String,
    /// Transaction date.
    #[serde(rename = "transactionDate")]
    pub transaction_date: String,
    /// Transaction price.
    #[serde(rename = "transactionPrice")]
    pub transaction_price: f64,
    /// Transaction code.
    #[serde(rename = "transactionCode")]
    pub transaction_code: String,
}

/// Insider transactions response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransactions {
    /// Symbol.
    pub symbol: String,
    /// Insider transaction data.
    pub data: Vec<InsiderTransaction>,
}

/// Insider sentiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderSentiment {
    /// Symbol.
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

/// Insider sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderSentimentData {
    /// Symbol.
    pub symbol: String,
    /// Insider sentiment data.
    pub data: Vec<InsiderSentiment>,
}
