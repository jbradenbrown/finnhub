//! Sentiment analysis endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{FilingSentiment, SocialSentiment},
};

/// Sentiment analysis endpoints.
pub struct SentimentEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> SentimentEndpoints<'a> {
    /// Create a new sentiment endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get social sentiment data.
    ///
    /// Returns social media sentiment data for a company.
    pub async fn social(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<SocialSentiment> {
        self.client
            .get(&format!(
                "/stock/social-sentiment?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get filing sentiment analysis.
    ///
    /// Analyze sentiment of company filings using NLP.
    ///
    /// # Arguments
    /// * `access_number` - Access number of the filing
    pub async fn filing(&self, access_number: &str) -> Result<FilingSentiment> {
        self.client
            .get(&format!("/stock/filings-sentiment?accessNumber={}", access_number))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientConfig, FinnhubClient, RateLimitStrategy};

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY")
            .unwrap_or_else(|_| "test_key".to_string());
        
        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_social_sentiment() {
        let client = test_client().await;
        let from = "2024-01-01";
        let to = "2024-01-31";
        let result = client.stock().social_sentiment("AAPL", from, to).await;
        
        assert!(result.is_ok(), "Failed to get social sentiment: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_filing_sentiment() {
        let client = test_client().await;
        // This requires a valid access number from an SEC filing
        // We'll use a known access number for testing
        let access_number = "0000320193-24-000123"; // Example Apple filing
        let result = client.stock().filing_sentiment(access_number).await;
        
        assert!(result.is_ok(), "Failed to get filing sentiment: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_social_sentiment_date_range() {
        let client = test_client().await;
        let from = "2024-06-01";
        let to = "2024-06-07"; // One week
        let result = client.stock().social_sentiment("TSLA", from, to).await;
        
        assert!(result.is_ok(), "Failed to get social sentiment with date range: {:?}", result.err());
    }
}