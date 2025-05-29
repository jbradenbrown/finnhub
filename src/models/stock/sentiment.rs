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