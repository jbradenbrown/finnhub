//! Corporate actions endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{Dividend, DividendsV2, IsinChange, StockSplit, SymbolChange},
};

/// Corporate actions endpoints.
pub struct CorporateActionsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> CorporateActionsEndpoints<'a> {
    /// Create a new corporate actions endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get dividends data.
    ///
    /// Returns dividend history with dates and amounts.
    pub async fn dividends(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<Dividend>> {
        self.client
            .get(&format!(
                "/stock/dividend?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get stock splits history.
    ///
    /// Returns stock split history with dates and split ratios.
    pub async fn splits(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<StockSplit>> {
        self.client
            .get(&format!(
                "/stock/split?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get dividends v2.
    ///
    /// Get dividend data including dividend yield and ex-dividend dates.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn dividends_v2(&self, symbol: &str) -> Result<DividendsV2> {
        self.client
            .get(&format!("/stock/dividend2?symbol={}", symbol))
            .await
    }

    /// Get a list of symbol changes for US-listed, EU-listed, NSE and ASX securities.
    ///
    /// Limited to 2000 events at a time.
    ///
    /// # Arguments
    /// * `from` - From date in `YYYY-MM-DD` format
    /// * `to` - To date in `YYYY-MM-DD` format
    pub async fn symbol_change(&self, from: &str, to: &str) -> Result<SymbolChange> {
        self.client
            .get(&format!("/ca/symbol-change?from={}&to={}", from, to))
            .await
    }

    /// Get a list of ISIN changes for EU-listed securities.
    ///
    /// Limited to 2000 events at a time.
    ///
    /// # Arguments
    /// * `from` - From date in `YYYY-MM-DD` format
    /// * `to` - To date in `YYYY-MM-DD` format
    pub async fn isin_change(&self, from: &str, to: &str) -> Result<IsinChange> {
        self.client
            .get(&format!("/ca/isin-change?from={}&to={}", from, to))
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
    async fn test_dividends() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client.stock().dividends("AAPL", from, to).await;

        // Just verify the API call completes successfully
        assert!(
            result.is_ok(),
            "Failed to get dividends: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_splits() {
        let client = test_client().await;
        let from = "2020-01-01";
        let to = "2024-12-31";
        let result = client.stock().splits("AAPL", from, to).await;

        // Just verify the API call completes successfully
        assert!(result.is_ok(), "Failed to get splits: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_dividends_v2() {
        let client = test_client().await;
        let result = client.stock().dividends_v2("MSFT").await;

        // Just verify the API call completes successfully
        assert!(
            result.is_ok(),
            "Failed to get dividends v2: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_symbol_change() {
        let client = test_client().await;
        let result = client
            .stock()
            .symbol_change("2024-01-01", "2024-06-30")
            .await;

        // Premium endpoint; allow 403/access errors but assert shape parses if available.
        if let Ok(changes) = result {
            for change in &changes.data {
                // Fields are optional — just confirm either at_date OR a symbol is set.
                let has_any = change.at_date.is_some()
                    || change.new_symbol.is_some()
                    || change.old_symbol.is_some();
                assert!(has_any, "expected at least one field to be present");
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_isin_change() {
        let client = test_client().await;
        let result = client.stock().isin_change("2024-01-01", "2024-06-30").await;

        if let Ok(changes) = result {
            for change in &changes.data {
                let has_any = change.at_date.is_some()
                    || change.new_isin.is_some()
                    || change.old_isin.is_some();
                assert!(has_any, "expected at least one field to be present");
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_no_dividends_company() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        // Tesla historically has not paid dividends
        let result = client.stock().dividends("TSLA", from, to).await;

        // Just verify the API call completes successfully (empty array is still success)
        assert!(
            result.is_ok(),
            "Failed to get dividends for TSLA: {:?}",
            result.err()
        );
    }
}
