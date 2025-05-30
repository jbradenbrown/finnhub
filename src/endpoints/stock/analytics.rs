//! Analytics and recommendations endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        PriceTarget, RecommendationTrend, RevenueBreakdown, UpgradeDowngrade,
    },
};

/// Analytics and recommendations endpoints.
pub struct AnalyticsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> AnalyticsEndpoints<'a> {
    /// Create a new analytics endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get latest price target consensus.
    pub async fn price_target(&self, symbol: &str) -> Result<PriceTarget> {
        self.client
            .get(&format!("/stock/price-target?symbol={}", symbol))
            .await
    }

    /// Get latest analyst recommendations.
    pub async fn recommendations(&self, symbol: &str) -> Result<Vec<RecommendationTrend>> {
        self.client
            .get(&format!("/stock/recommendation?symbol={}", symbol))
            .await
    }

    /// Get revenue breakdown data.
    ///
    /// Returns revenue breakdown by business segment, product, or geography.
    pub async fn revenue_breakdown(&self, symbol: &str) -> Result<RevenueBreakdown> {
        self.client
            .get(&format!("/stock/revenue-breakdown?symbol={}", symbol))
            .await
    }

    /// Get stock upgrades and downgrades.
    ///
    /// Returns analyst upgrades and downgrades for a company.
    pub async fn upgrade_downgrade(
        &self,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<UpgradeDowngrade>> {
        let mut params = Vec::new();
        if let Some(symbol) = symbol {
            params.push(format!("symbol={}", symbol));
        }
        if let Some(from) = from {
            params.push(format!("from={}", from));
        }
        if let Some(to) = to {
            params.push(format!("to={}", to));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };

        self.client
            .get(&format!("/stock/upgrade-downgrade{}", query))
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
    async fn test_price_target() {
        let client = test_client().await;
        let result = client.stock().price_target("AAPL").await;
        
        assert!(result.is_ok(), "Failed to get price target: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_recommendations() {
        let client = test_client().await;
        let result = client.stock().recommendations("AAPL").await;
        
        assert!(result.is_ok(), "Failed to get recommendations: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_revenue_breakdown() {
        let client = test_client().await;
        let result = client.stock().revenue_breakdown("AAPL").await;
        
        // Revenue breakdown might not be available for all companies
        assert!(result.is_ok(), "Failed to get revenue breakdown: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_upgrade_downgrade() {
        let client = test_client().await;
        
        // Test with symbol
        let result = client.stock().upgrade_downgrade(Some("AAPL"), None, None).await;
        
        assert!(result.is_ok(), "Failed to get upgrade/downgrade: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_upgrade_downgrade_with_dates() {
        let client = test_client().await;
        
        // Test with date range
        let from = "2024-01-01";
        let to = "2024-12-31";
        let result = client.stock().upgrade_downgrade(Some("AAPL"), Some(from), Some(to)).await;
        
        assert!(result.is_ok(), "Failed to get upgrade/downgrade with dates: {:?}", result.err());
    }
}