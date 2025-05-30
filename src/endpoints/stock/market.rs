//! Market data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{InvestmentTheme, MarketHoliday, MarketStatus},
};

/// Market data endpoints.
pub struct MarketEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> MarketEndpoints<'a> {
    /// Create a new market endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get current market status.
    ///
    /// Returns whether the exchange is open or closed.
    pub async fn status(&self, exchange: &str) -> Result<MarketStatus> {
        self.client
            .get(&format!("/stock/market-status?exchange={}", exchange))
            .await
    }

    /// Get market holidays.
    ///
    /// Returns a list of holidays for global exchanges.
    ///
    /// # Arguments
    /// * `exchange` - Exchange code
    pub async fn holiday(&self, exchange: &str) -> Result<MarketHoliday> {
        self.client
            .get(&format!("/stock/market-holiday?exchange={}", exchange))
            .await
    }

    /// Get investment theme portfolio.
    ///
    /// Returns portfolios of different investment themes that are changing our life and are the way of the future.
    ///
    /// # Arguments
    /// * `theme` - Investment theme (e.g., "financialExchangesData", "futureFood")
    pub async fn investment_theme(&self, theme: &str) -> Result<InvestmentTheme> {
        self.client
            .get(&format!("/stock/investment-theme?theme={}", theme))
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
    async fn test_market_status() {
        let client = test_client().await;
        let result = client.stock().market_status("US").await;
        
        assert!(result.is_ok(), "Failed to get market status: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_market_holiday() {
        let client = test_client().await;
        let result = client.stock().market_holiday("US").await;
        
        assert!(result.is_ok(), "Failed to get market holidays: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_investment_theme() {
        let client = test_client().await;
        let result = client.stock().investment_theme("financialExchangesData").await;
        
        assert!(result.is_ok(), "Failed to get investment theme: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_investment_theme_future_food() {
        let client = test_client().await;
        let result = client.stock().investment_theme("futureFood").await;
        
        assert!(result.is_ok(), "Failed to get future food theme: {:?}", result.err());
    }
}