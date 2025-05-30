//! Financial data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        BasicFinancials, Earnings, FinancialStatements, FinancialsAsReported, StatementFrequency,
        StatementType,
    },
};

/// Financial data endpoints.
pub struct FinancialsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> FinancialsEndpoints<'a> {
    /// Create a new financials endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get standardized financial statements.
    ///
    /// Get balance sheet, income statement, or cash flow for global companies.
    pub async fn statements(
        &self,
        symbol: &str,
        statement: StatementType,
        frequency: StatementFrequency,
    ) -> Result<FinancialStatements> {
        self.client
            .get(&format!(
                "/stock/financials?symbol={}&statement={}&freq={}",
                symbol, statement, frequency
            ))
            .await
    }

    /// Get basic financials metrics.
    ///
    /// Returns key metrics such as P/E ratio, market cap, 52-week high/low, etc.
    pub async fn metrics(&self, symbol: &str) -> Result<BasicFinancials> {
        self.client
            .get(&format!("/stock/metric?symbol={}&metric=all", symbol))
            .await
    }

    /// Get company earnings.
    pub async fn earnings(&self, symbol: &str, limit: Option<i64>) -> Result<Vec<Earnings>> {
        let url = if let Some(limit) = limit {
            format!("/stock/earnings?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/earnings?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get financials as reported.
    ///
    /// Returns detailed financial statements as reported to the SEC.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional if using CIK or access number)
    /// * `cik` - CIK number (optional)
    /// * `access_number` - Access number of a specific report (optional)
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn as_reported(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        freq: Option<&str>,
    ) -> Result<FinancialsAsReported> {
        let mut params = vec![];

        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = cik {
            params.push(format!("cik={}", c));
        }
        if let Some(a) = access_number {
            params.push(format!("accessNumber={}", a));
        }
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = if params.is_empty() {
            String::from("/stock/financials-reported")
        } else {
            format!("/stock/financials-reported?{}", params.join("&"))
        };

        self.client.get(&query).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::stock::{StatementFrequency, StatementType},
        ClientConfig, FinnhubClient, RateLimitStrategy,
    };

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY").unwrap_or_else(|_| "test_key".to_string());

        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_financial_statements_balance_sheet() {
        let client = test_client().await;
        let result = client
            .stock()
            .financials(
                "AAPL",
                StatementType::BalanceSheet,
                StatementFrequency::Annual,
            )
            .await;

        assert!(
            result.is_ok(),
            "Failed to get balance sheet: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_financial_statements_income() {
        let client = test_client().await;
        let result = client
            .stock()
            .financials(
                "MSFT",
                StatementType::IncomeStatement,
                StatementFrequency::Quarterly,
            )
            .await;

        assert!(
            result.is_ok(),
            "Failed to get income statement: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_financial_statements_cash_flow() {
        let client = test_client().await;
        let result = client
            .stock()
            .financials("GOOGL", StatementType::CashFlow, StatementFrequency::Annual)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get cash flow statement: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_basic_financials_metrics() {
        let client = test_client().await;
        let result = client.stock().metrics("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get basic financials metrics: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_earnings() {
        let client = test_client().await;
        let result = client.stock().earnings("AAPL", Some(4)).await;

        assert!(result.is_ok(), "Failed to get earnings: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_financials_as_reported() {
        let client = test_client().await;
        let result = client
            .stock()
            .financials_reported(Some("AAPL"), None, None, Some("annual"))
            .await;

        assert!(
            result.is_ok(),
            "Failed to get financials as reported: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_earnings_no_limit() {
        let client = test_client().await;
        let result = client.stock().earnings("MSFT", None).await;

        assert!(
            result.is_ok(),
            "Failed to get earnings without limit: {:?}",
            result.err()
        );
    }
}
