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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClientConfig, FinnhubClient, RateLimitStrategy};

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY").unwrap_or_else(|_| "test_key".to_string());

        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_market_news() {
        let client = test_client().await;
        let result = client.news().market_news(NewsCategory::General, None).await;
        assert!(
            result.is_ok(),
            "Failed to get market news: {:?}",
            result.err()
        );

        let news = result.unwrap();
        assert!(!news.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_company_news() {
        let client = test_client().await;
        let from = "2024-01-01";
        let to = "2024-01-31";
        let result = client.news().company_news("AAPL", from, to).await;
        assert!(
            result.is_ok(),
            "Failed to get company news: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_news_sentiment() {
        let client = test_client().await;
        let result = client.news().news_sentiment("AAPL").await;
        assert!(
            result.is_ok(),
            "Failed to get news sentiment: {:?}",
            result.err()
        );
    }
}
