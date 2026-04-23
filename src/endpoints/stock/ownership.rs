//! Ownership data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        FundOwnership, InstitutionalOwnership, InstitutionalPortfolio, InstitutionalProfile,
        OwnershipData,
    },
};

/// Ownership data endpoints.
pub struct OwnershipEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> OwnershipEndpoints<'a> {
    /// Create a new ownership endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get company ownership data.
    ///
    /// Returns a list of company shareholders/owners.
    pub async fn institutional(&self, symbol: &str, limit: Option<i64>) -> Result<OwnershipData> {
        let url = if let Some(limit) = limit {
            format!("/stock/ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get fund ownership.
    ///
    /// Get a list of funds that hold shares of a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `limit` - Limit number of results (optional)
    pub async fn fund(&self, symbol: &str, limit: Option<i64>) -> Result<FundOwnership> {
        let url = if let Some(limit) = limit {
            format!("/stock/fund-ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/fund-ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get a list of well-known institutional investors.
    ///
    /// # Arguments
    /// * `cik` - Filter by CIK (optional; leave `None` for the full list)
    pub async fn institutional_profile(
        &self,
        cik: Option<&str>,
    ) -> Result<InstitutionalProfile> {
        let url = match cik {
            Some(c) => format!("/institutional/profile?cik={}", c),
            None => "/institutional/profile".to_string(),
        };
        self.client.get(&url).await
    }

    /// Get the holdings/portfolio of an institutional investor from 13-F filings.
    ///
    /// Limited to one year of data at a time.
    ///
    /// # Arguments
    /// * `cik` - Fund's CIK
    /// * `from` - From date in `YYYY-MM-DD` format
    /// * `to` - To date in `YYYY-MM-DD` format
    pub async fn institutional_portfolio(
        &self,
        cik: &str,
        from: &str,
        to: &str,
    ) -> Result<InstitutionalPortfolio> {
        self.client
            .get(&format!(
                "/institutional/portfolio?cik={}&from={}&to={}",
                cik, from, to
            ))
            .await
    }

    /// Get institutional investors' positions in a particular stock over time.
    ///
    /// Data is sourced from 13-F filings; limited to one year of data at a time.
    /// Pass either a `symbol` or a `cusip` (or both).
    ///
    /// # Arguments
    /// * `symbol` - Filter by symbol
    /// * `cusip` - Filter by CUSIP (use empty string to omit)
    /// * `from` - From date in `YYYY-MM-DD` format
    /// * `to` - To date in `YYYY-MM-DD` format
    pub async fn institutional_ownership(
        &self,
        symbol: &str,
        cusip: &str,
        from: &str,
        to: &str,
    ) -> Result<InstitutionalOwnership> {
        self.client
            .get(&format!(
                "/institutional/ownership?symbol={}&cusip={}&from={}&to={}",
                symbol, cusip, from, to
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
    async fn test_institutional_ownership() {
        let client = test_client().await;
        let result = client.stock().ownership("AAPL", None).await;

        assert!(
            result.is_ok(),
            "Failed to get institutional ownership: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_ownership_with_limit() {
        let client = test_client().await;
        let limit = 10;
        let result = client.stock().ownership("MSFT", Some(limit)).await;

        assert!(
            result.is_ok(),
            "Failed to get institutional ownership with limit: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_fund_ownership() {
        let client = test_client().await;
        let result = client.stock().fund_ownership("AAPL", None).await;

        assert!(
            result.is_ok(),
            "Failed to get fund ownership: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_fund_ownership_with_limit() {
        let client = test_client().await;
        let limit = 5;
        let result = client.stock().fund_ownership("GOOGL", Some(limit)).await;

        assert!(
            result.is_ok(),
            "Failed to get fund ownership with limit: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_profile_list() {
        let client = test_client().await;
        let result = client.stock().institutional_profile(None).await;

        assert!(
            result.is_ok(),
            "Failed to get institutional profile list: {:?}",
            result.err()
        );

        if let Ok(profile) = result {
            for entry in &profile.data {
                assert!(!entry.cik.is_empty());
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_portfolio() {
        let client = test_client().await;
        // Berkshire Hathaway CIK
        let result = client
            .stock()
            .institutional_portfolio("1067983", "2024-01-01", "2024-06-30")
            .await;

        assert!(
            result.is_ok(),
            "Failed to get institutional portfolio: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_ownership_13f() {
        let client = test_client().await;
        let result = client
            .stock()
            .institutional_ownership("AAPL", "", "2024-01-01", "2024-06-30")
            .await;

        assert!(
            result.is_ok(),
            "Failed to get institutional ownership 13F: {:?}",
            result.err()
        );

        if let Ok(ownership) = result {
            assert_eq!(ownership.symbol, "AAPL");
        }
    }
}
