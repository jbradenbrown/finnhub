//! News endpoints.

use crate::{client::FinnhubClient, error::Result, models::news::*};

/// News-related API endpoints.
pub struct NewsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> NewsEndpoints<'a> {
    /// Create a new news endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get market news.
    ///
    /// Get latest market news by category.
    pub async fn market_news(
        &self,
        category: NewsCategory,
        min_id: Option<i64>,
    ) -> Result<Vec<MarketNews>> {
        let url = if let Some(min_id) = min_id {
            format!("/news?category={}&minId={}", category, min_id)
        } else {
            format!("/news?category={}", category)
        };
        self.client.get(&url).await
    }

    /// Get company news.
    ///
    /// Get latest news for a specific company.
    pub async fn company_news(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<Vec<CompanyNews>> {
        self.client
            .get(&format!(
                "/company-news?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get news sentiment.
    ///
    /// Get sentiment analysis for company news.
    pub async fn news_sentiment(&self, symbol: &str) -> Result<NewsSentiment> {
        self.client
            .get(&format!("/news-sentiment?symbol={}", symbol))
            .await
    }
}
