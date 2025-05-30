//! SEC filings and document endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        EarningsCallLive, EarningsCallTranscript, EarningsCallTranscriptsList, Filing,
        InternationalFiling, InvestorPresentations, SimilarityIndex,
    },
};

/// SEC filings and document endpoints.
pub struct FilingsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> FilingsEndpoints<'a> {
    /// Create a new filings endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get SEC filings.
    ///
    /// List company's SEC filings. You can filter by symbol, CIK, access number, form type, and date range.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `cik` - CIK number (optional)
    /// * `access_number` - Access number of a specific report (optional)
    /// * `form` - Filter by form type (optional)
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    pub async fn sec(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        form: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<Filing>> {
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
        if let Some(f) = form {
            params.push(format!("form={}", f));
        }
        if let Some(from_date) = from {
            params.push(format!("from={}", from_date));
        }
        if let Some(to_date) = to {
            params.push(format!("to={}", to_date));
        }

        let query = if params.is_empty() {
            String::from("/stock/filings")
        } else {
            format!("/stock/filings?{}", params.join("&"))
        };

        self.client.get(&query).await
    }

    /// Get international filings.
    ///
    /// List filings for international companies. Limit to 500 documents at a time.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `country` - Filter by country using country's 2-letter code (optional)
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    pub async fn international(
        &self,
        symbol: Option<&str>,
        country: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<InternationalFiling>> {
        let mut params = vec![];

        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = country {
            params.push(format!("country={}", c));
        }
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }

        let query = if params.is_empty() {
            String::from("/stock/international-filings")
        } else {
            format!("/stock/international-filings?{}", params.join("&"))
        };

        self.client.get(&query).await
    }

    /// Get earnings call transcripts.
    ///
    /// Get earnings call transcripts, audio and participants' list.
    ///
    /// # Arguments
    /// * `id` - Transcript's id obtained with Transcripts List endpoint
    pub async fn transcript(&self, id: &str) -> Result<EarningsCallTranscript> {
        self.client
            .get(&format!("/stock/transcripts?id={}", id))
            .await
    }

    /// Get earnings call transcripts list.
    ///
    /// List earnings call transcripts' metadata.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn transcripts_list(&self, symbol: &str) -> Result<EarningsCallTranscriptsList> {
        self.client
            .get(&format!("/stock/transcripts/list?symbol={}", symbol))
            .await
    }

    /// Get earnings call live events.
    ///
    /// Get upcoming earnings call events that support live audio streaming.
    ///
    /// # Arguments
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn earnings_call_live(&self, from: &str, to: &str) -> Result<EarningsCallLive> {
        self.client
            .get(&format!(
                "/stock/earnings-call-live?from={}&to={}",
                from, to
            ))
            .await
    }

    /// Get investor presentations.
    ///
    /// List all investor presentations for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn presentations(&self, symbol: &str) -> Result<InvestorPresentations> {
        self.client
            .get(&format!("/stock/presentation?symbol={}", symbol))
            .await
    }

    /// Get document similarity index.
    ///
    /// Compare company filings and estimate percentage of changes.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `cik` - CIK number (optional)
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn similarity_index(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        freq: Option<&str>,
    ) -> Result<SimilarityIndex> {
        let mut params = vec![];

        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = cik {
            params.push(format!("cik={}", c));
        }
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        if params.is_empty() {
            return Err(crate::Error::InvalidRequest(
                "At least one of symbol or cik must be provided".to_string(),
            ));
        }

        let query = format!("/stock/similarity-index?{}", params.join("&"));
        self.client.get(&query).await
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
    async fn test_sec_filings() {
        let client = test_client().await;
        let result = client
            .stock()
            .sec_filings(Some("AAPL"), None, None, None, None, None)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get SEC filings: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sec_filings_with_form_filter() {
        let client = test_client().await;
        let result = client
            .stock()
            .sec_filings(Some("MSFT"), None, None, Some("10-K"), None, None)
            .await;

        assert!(
            result.is_ok(),
            "Failed to get SEC filings with form filter: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sec_filings_with_date_range() {
        let client = test_client().await;
        let from = "2023-01-01";
        let to = "2023-12-31";
        let result = client
            .stock()
            .sec_filings(Some("GOOGL"), None, None, None, Some(from), Some(to))
            .await;

        assert!(
            result.is_ok(),
            "Failed to get SEC filings with date range: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_international_filings() {
        let client = test_client().await;
        // Test with a company that has international filings
        let result = client
            .stock()
            .international_filings(
                Some("NVO"), // Novo Nordisk (Danish company)
                None,
                None,
                None,
            )
            .await;

        assert!(
            result.is_ok(),
            "Failed to get international filings: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_transcripts_list() {
        let client = test_client().await;
        let result = client.stock().transcripts_list("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get transcripts list: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_earnings_call_live() {
        let client = test_client().await;
        // Look for earnings calls in the next 30 days
        let from = chrono::Local::now().format("%Y-%m-%d").to_string();
        let to = (chrono::Local::now() + chrono::Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();

        let result = client.stock().earnings_call_live(&from, &to).await;

        assert!(
            result.is_ok(),
            "Failed to get earnings call live: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_presentations() {
        let client = test_client().await;
        let result = client.stock().presentations("AAPL").await;

        assert!(
            result.is_ok(),
            "Failed to get presentations: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_similarity_index() {
        let client = test_client().await;
        let result = client
            .stock()
            .similarity_index(Some("AAPL"), None, Some("annual"))
            .await;

        assert!(
            result.is_ok(),
            "Failed to get similarity index: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_similarity_index_error() {
        let client = test_client().await;
        // Should fail when neither symbol nor cik is provided
        let result = client.stock().similarity_index(None, None, None).await;

        assert!(
            result.is_err(),
            "Should fail when neither symbol nor cik is provided"
        );
    }
}
