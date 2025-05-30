//! News-related data models.

use serde::{Deserialize, Serialize};

/// Market news item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketNews {
    /// News category.
    pub category: String,
    /// Published datetime (UNIX timestamp).
    pub datetime: i64,
    /// News headline.
    pub headline: String,
    /// News ID.
    pub id: i64,
    /// Thumbnail image URL.
    pub image: String,
    /// Related symbol (if any).
    pub related: String,
    /// News source.
    pub source: String,
    /// News summary.
    pub summary: String,
    /// News URL.
    pub url: String,
}

/// Company news item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyNews {
    /// News category.
    pub category: String,
    /// Published datetime (UNIX timestamp).
    pub datetime: i64,
    /// News headline.
    pub headline: String,
    /// News ID.
    pub id: i64,
    /// Thumbnail image URL.
    pub image: String,
    /// Related symbol.
    pub related: String,
    /// News source.
    pub source: String,
    /// News summary.
    pub summary: String,
    /// News URL.
    pub url: String,
}

/// News sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsSentiment {
    /// Company symbol.
    pub symbol: String,
    /// Buzz metrics.
    pub buzz: NewsBuzz,
    /// Company news score.
    pub company_news_score: f64,
    /// Sector average bullishness.
    #[serde(rename = "sectorAverageBullishPercent")]
    pub sector_average_bullishness: f64,
    /// Sector average news score.
    pub sector_average_news_score: f64,
    /// Sentiment.
    pub sentiment: SentimentData,
}

/// News buzz metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsBuzz {
    /// Articles in the past week.
    pub articles_in_last_week: i64,
    /// Buzz score.
    pub buzz: f64,
    /// Weekly average.
    pub weekly_average: f64,
}

/// Sentiment data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentimentData {
    /// Bearish percent.
    pub bearish_percent: f64,
    /// Bullish percent.
    pub bullish_percent: f64,
}

/// News category.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NewsCategory {
    /// General news.
    #[serde(rename = "general")]
    General,
    /// Forex news.
    #[serde(rename = "forex")]
    Forex,
    /// Crypto news.
    #[serde(rename = "crypto")]
    Crypto,
    /// Merger news.
    #[serde(rename = "merger")]
    Merger,
}

impl std::fmt::Display for NewsCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewsCategory::General => write!(f, "general"),
            NewsCategory::Forex => write!(f, "forex"),
            NewsCategory::Crypto => write!(f, "crypto"),
            NewsCategory::Merger => write!(f, "merger"),
        }
    }
}
