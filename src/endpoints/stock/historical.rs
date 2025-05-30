//! Historical data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        HistoricalESG, HistoricalEmployeeCount, HistoricalMarketCapData, HistoricalNBBO,
    },
};

/// Historical data endpoints.
pub struct HistoricalEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> HistoricalEndpoints<'a> {
    /// Create a new historical endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get historical market capitalization data.
    ///
    /// Returns historical market cap values for a given date range.
    pub async fn market_cap(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalMarketCapData> {
        self.client
            .get(&format!(
                "/stock/historical-market-cap?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get historical employee count data.
    ///
    /// Returns historical employee count for a given date range.
    pub async fn employee_count(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalEmployeeCount> {
        self.client
            .get(&format!(
                "/stock/historical-employee-count?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get historical ESG (Environmental, Social, Governance) scores.
    ///
    /// Returns historical ESG scores for a given date range.
    pub async fn esg(&self, symbol: &str, from: &str, to: &str) -> Result<HistoricalESG> {
        self.client
            .get(&format!(
                "/stock/historical-esg?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get historical NBBO (National Best Bid and Offer) data.
    ///
    /// Returns historical best bid and offer for US stocks, LSE, TSX, Euronext and Deutsche Borse.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `date` - Date in YYYY-MM-DD format
    /// * `limit` - Limit number of ticks returned (max 25000)
    /// * `skip` - Number of ticks to skip
    pub async fn nbbo(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<HistoricalNBBO> {
        self.client
            .get(&format!(
                "/stock/bbo?symbol={}&date={}&limit={}&skip={}",
                symbol, date, limit, skip
            ))
            .await
    }
}

#[cfg(test)]
mod tests {
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
    async fn test_historical_market_cap() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client.stock().historical_market_cap("AAPL", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get historical market cap: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_historical_employee_count() {
        let client = test_client().await;
        let from = "2020-01-01";
        let to = "2023-12-31";
        let result = client
            .stock()
            .historical_employee_count("MSFT", from, to)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get historical employee count: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_historical_esg() {
        let client = test_client().await;
        let from = "2020-01-01";
        let to = "2023-12-31";
        let result = client.stock().historical_esg("AAPL", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get historical ESG: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_historical_nbbo() {
        let client = test_client().await;
        let date = "2024-01-02"; // A specific trading day
        let limit = 100;
        let skip = 0;
        let result = client
            .stock()
            .historical_nbbo("AAPL", date, limit, skip)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get historical NBBO: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_historical_nbbo_pagination() {
        let client = test_client().await;
        let date = "2024-01-02"; // A specific trading day
        let limit = 50;
        let skip = 100;
        let result = client
            .stock()
            .historical_nbbo("AAPL", date, limit, skip)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get historical NBBO with pagination: {:?}",
            result.err()
        );
    }
}
