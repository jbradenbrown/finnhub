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
    pub async fn institutional_profile(&self, cik: Option<&str>) -> Result<InstitutionalProfile> {
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
    /// `symbol` is primary; `cusip` is an optional secondary filter.
    ///
    /// # Arguments
    /// * `symbol` - Filter by symbol
    /// * `cusip` - Optional CUSIP filter
    /// * `from` - From date in `YYYY-MM-DD` format
    /// * `to` - To date in `YYYY-MM-DD` format
    pub async fn institutional_ownership(
        &self,
        symbol: &str,
        cusip: Option<&str>,
        from: &str,
        to: &str,
    ) -> Result<InstitutionalOwnership> {
        // The API always expects a `cusip` query parameter; pass empty when
        // the caller hasn't supplied one.
        let cusip = cusip.unwrap_or("");
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

    /// Fixture-based parse test for `InstitutionalPortfolio`.
    ///
    /// The nested position struct has several camelCase renames
    /// (`noVoting`, `sharedVoting`, `soleVoting`, `putCall`) that are
    /// easy to get wrong. This test deserializes a trimmed real API
    /// response and asserts the field values round-trip correctly, so
    /// a rename regression is caught even without a premium API key.
    #[test]
    fn test_institutional_portfolio_fixture_parses() {
        use crate::models::stock::InstitutionalPortfolio;

        // Trimmed from a real /institutional/portfolio response.
        let sample = r#"{
          "cik": "1000097",
          "name": "KINGDON CAPITAL MANAGEMENT, L.L.C.",
          "data": [
            {
              "filingDate": "2022-06-30",
              "reportDate": "2022-06-30",
              "portfolio": [
                {
                  "change": -41600,
                  "cusip": "002824100",
                  "name": "ABBOTT LABS",
                  "noVoting": 0,
                  "percentage": 0.41,
                  "putCall": "",
                  "share": 29000000,
                  "sharedVoting": 1000,
                  "soleVoting": 41600,
                  "symbol": "ABT",
                  "value": 1150430000.0
                }
              ]
            }
          ]
        }"#;

        let parsed: InstitutionalPortfolio = serde_json::from_str(sample).unwrap();
        assert_eq!(parsed.cik, "1000097");
        assert_eq!(parsed.name.as_deref(), Some("KINGDON CAPITAL MANAGEMENT, L.L.C."));
        assert_eq!(parsed.data.len(), 1);

        let group = &parsed.data[0];
        assert_eq!(group.filing_date, "2022-06-30");
        assert_eq!(group.report_date, "2022-06-30");
        assert_eq!(group.portfolio.len(), 1);

        let pos = &group.portfolio[0];
        assert_eq!(pos.symbol.as_deref(), Some("ABT"));
        assert_eq!(pos.change, Some(-41600));
        // Verify the camelCase renames actually landed on the right fields.
        assert_eq!(pos.no_voting, Some(0));
        assert_eq!(pos.shared_voting, Some(1000));
        assert_eq!(pos.sole_voting, Some(41600));
        assert_eq!(pos.put_call.as_deref(), Some(""));
        assert_eq!(pos.share, Some(29000000));
        assert_eq!(pos.value, Some(1150430000.0));
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_institutional_ownership_13f() {
        let client = test_client().await;
        let result = client
            .stock()
            .institutional_ownership("AAPL", None, "2024-01-01", "2024-06-30")
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
