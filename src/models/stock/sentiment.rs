//! Sentiment analysis models.

use serde::{Deserialize, Serialize};

/// Social sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSentimentData {
    /// At date.
    #[serde(rename = "atTime")]
    pub at_time: String,
    /// Mention count.
    pub mention: i64,
    /// Positive mention count.
    #[serde(rename = "positiveMention")]
    pub positive_mention: i64,
    /// Negative mention count.
    #[serde(rename = "negativeMention")]
    pub negative_mention: i64,
    /// Positive score.
    #[serde(rename = "positiveScore")]
    pub positive_score: f64,
    /// Negative score.
    #[serde(rename = "negativeScore")]
    pub negative_score: f64,
    /// Overall score.
    pub score: f64,
}

/// Social sentiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSentiment {
    /// Symbol.
    pub symbol: String,
    /// Social sentiment data.
    pub data: Vec<SocialSentimentData>,
    /// Reddit data.
    pub reddit: Option<Vec<SocialSentimentData>>,
    /// Twitter data.
    pub twitter: Option<Vec<SocialSentimentData>>,
}

/// Filing sentiment analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilingSentiment {
    /// Access number.
    #[serde(rename = "accessNumber")]
    pub access_number: String,
    /// Symbol.
    pub symbol: String,
    /// CIK.
    pub cik: String,
    /// Sentiment scores.
    pub sentiment: SentimentScores,
}

/// Sentiment scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentScores {
    /// Percentage of negative words.
    pub negative: f64,
    /// Percentage of positive words.
    pub positive: f64,
    /// Polarity score.
    pub polarity: f64,
    /// Percentage of litigious words.
    pub litigious: f64,
    /// Percentage of uncertainty words.
    pub uncertainty: f64,
    /// Percentage of constraining words.
    pub constraining: f64,
    /// Percentage of modal-weak words.
    #[serde(rename = "modal-weak")]
    pub modal_weak: f64,
    /// Percentage of modal-strong words.
    #[serde(rename = "modal-strong")]
    pub modal_strong: f64,
    /// Percentage of modal-moderate words.
    #[serde(rename = "modal-moderate")]
    pub modal_moderate: f64,
}
