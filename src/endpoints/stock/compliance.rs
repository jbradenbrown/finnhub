//! Compliance and regulatory endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        CompanyExecutives, CongressionalTrading, ESGScore, Lobbying, SupplyChainData, USASpending,
        USPTOPatents, VisaApplications,
    },
};

/// Compliance and regulatory endpoints.
pub struct ComplianceEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> ComplianceEndpoints<'a> {
    /// Create a new compliance endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get company executives.
    ///
    /// Returns a list of company's executives and board members with compensation data.
    pub async fn executives(&self, symbol: &str) -> Result<CompanyExecutives> {
        self.client
            .get(&format!("/stock/executive?symbol={}", symbol))
            .await
    }

    /// Get congressional trading data.
    ///
    /// Returns trading activity by US congress members for a symbol.
    pub async fn congressional_trading(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<CongressionalTrading> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }

        let query = format!("/stock/congressional-trading?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get lobbying data.
    ///
    /// Returns lobbying activities for a company.
    pub async fn lobbying(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Lobbying> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }

        let query = format!("/stock/lobbying?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get USA spending data.
    ///
    /// Returns government contracts awarded to a company.
    pub async fn usa_spending(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<USASpending> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }

        let query = format!("/stock/usa-spending?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get current ESG scores.
    ///
    /// Get current ESG (Environmental, Social, Governance) scores for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn esg(&self, symbol: &str) -> Result<ESGScore> {
        self.client
            .get(&format!("/stock/esg?symbol={}", symbol))
            .await
    }

    /// Get supply chain relationships.
    ///
    /// Returns companies in the supply chain (suppliers and customers).
    pub async fn supply_chain(&self, symbol: &str) -> Result<SupplyChainData> {
        self.client
            .get(&format!("/stock/supply-chain?symbol={}", symbol))
            .await
    }

    /// Get USPTO patent applications.
    ///
    /// List USPTO patent applications for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn uspto_patents(&self, symbol: &str, from: &str, to: &str) -> Result<USPTOPatents> {
        self.client
            .get(&format!(
                "/stock/uspto-patent?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get visa applications.
    ///
    /// List H1B visa applications for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn visa_applications(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<VisaApplications> {
        self.client
            .get(&format!(
                "/stock/visa-application?symbol={}&from={}&to={}",
                symbol, from, to
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
    async fn test_executives() {
        let client = test_client().await;
        let result = client.stock().executives("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get executives: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_congressional_trading() {
        let client = test_client().await;
        let result = client
            .stock()
            .congressional_trading("AAPL", None, None)
            .await;

        // Congressional trading data might not be available for all companies
        assert!(
            result.is_ok(),
            "Failed to get congressional trading: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_lobbying() {
        let client = test_client().await;
        let result = client.stock().lobbying("AAPL", None, None).await;

        // Lobbying data might not be available for all companies
        assert!(result.is_ok(), "Failed to get lobbying: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_usa_spending() {
        let client = test_client().await;
        let result = client.stock().usa_spending("LMT", None, None).await; // Lockheed Martin likely has govt contracts

        assert!(
            result.is_ok(),
            "Failed to get USA spending: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_esg() {
        let client = test_client().await;
        let result = client.stock().esg("AAPL").await;

        assert!(result.is_ok(), "Failed to get ESG: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_supply_chain() {
        let client = test_client().await;
        let result = client.stock().supply_chain("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get supply chain: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_uspto_patents() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client.stock().uspto_patents("AAPL", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get USPTO patents: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_visa_applications() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client.stock().visa_applications("GOOGL", from, to).await;

        assert!(
            result.is_ok(),
            "Failed to get visa applications: {:?}",
            result.err()
        );
    }
}
