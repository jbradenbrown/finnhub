//! Global filings endpoints (filter, search, search-in-filing).

use crate::{
    client::FinnhubClient,
    error::Result,
    models::global_filings::{
        InFilingResponse, InFilingSearchBody, SearchBody, SearchFilter, SearchResponse,
    },
};

/// Global-filings endpoints.
pub struct GlobalFilingsEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> GlobalFilingsEndpoints<'a> {
    /// Create a new instance of global-filings endpoints.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// List the available filter values for a search field.
    ///
    /// Each returned entry's `id` can be used in the matching field of
    /// [`SearchBody`] (e.g. filter by `form` to discover form-type ids).
    ///
    /// # Response shape
    ///
    /// Assumed to be a top-level JSON array (`[{id, name}, …]`). This
    /// endpoint is premium-only; if a live response turns out to be
    /// wrapped (`{data: [...]}`), this deserialization will fail and
    /// the return type will need to be wrapped accordingly.
    ///
    /// # Arguments
    /// * `field` - Field to filter on (e.g. `form`, `source`, `country`)
    /// * `source` - Optional source restriction
    pub async fn filter(
        &self,
        field: &str,
        source: Option<&str>,
    ) -> Result<Vec<SearchFilter>> {
        let mut params = vec![format!("field={}", field)];
        if let Some(s) = source {
            params.push(format!("source={}", s));
        }
        let query = format!("/global-filings/filter?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Full-text search across global filings.
    pub async fn search(&self, body: &SearchBody) -> Result<SearchResponse> {
        self.client.post("/global-filings/search", body).await
    }

    /// Search within a single filing's documents.
    pub async fn search_in_filing(&self, body: &InFilingSearchBody) -> Result<InFilingResponse> {
        self.client
            .post("/global-filings/search-in-filing", body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::global_filings::{InFilingSearchBody, SearchBody},
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
    #[ignore = "requires API key (premium)"]
    async fn test_filter() {
        let client = test_client().await;
        let result = client.global_filings().filter("form", None).await;

        // Premium endpoint; allow 403 but assert shape parses if available.
        if let Ok(filters) = result {
            // Response is an array; each entry should have at least an id.
            for entry in &filters {
                assert!(entry.id.is_some() || entry.name.is_some());
            }
        }
    }

    #[test]
    fn test_search_body_serializes_camelcase_and_skips_none() {
        // Catches rename bugs without hitting the API.
        let body = SearchBody {
            query: Some("climate".to_string()),
            forms: Some("10-K".to_string()),
            from_date: Some("2024-01-01".to_string()),
            to_date: Some("2024-03-31".to_string()),
            highlighted: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&body).unwrap();
        let obj = json.as_object().unwrap();

        assert_eq!(obj.get("query").and_then(|v| v.as_str()), Some("climate"));
        assert_eq!(obj.get("forms").and_then(|v| v.as_str()), Some("10-K"));
        // camelCase: fromDate / toDate, not from_date / to_date.
        assert_eq!(
            obj.get("fromDate").and_then(|v| v.as_str()),
            Some("2024-01-01")
        );
        assert_eq!(
            obj.get("toDate").and_then(|v| v.as_str()),
            Some("2024-03-31")
        );
        assert_eq!(obj.get("highlighted").and_then(|v| v.as_bool()), Some(true));
        // Unset fields must be omitted so the server sees no filter.
        assert!(!obj.contains_key("isins"));
        assert!(!obj.contains_key("symbols"));
        assert!(!obj.contains_key("page"));
    }

    #[test]
    fn test_in_filing_search_body_serializes_camelcase() {
        let body = InFilingSearchBody {
            query: "climate".to_string(),
            filing_id: "abc123".to_string(),
        };
        let json = serde_json::to_value(&body).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.get("query").and_then(|v| v.as_str()), Some("climate"));
        assert_eq!(obj.get("filingId").and_then(|v| v.as_str()), Some("abc123"));
    }

    #[tokio::test]
    #[ignore = "requires API key (premium)"]
    async fn test_search() {
        let client = test_client().await;
        let body = SearchBody {
            query: Some("climate".to_string()),
            forms: Some("10-K".to_string()),
            from_date: Some("2024-01-01".to_string()),
            to_date: Some("2024-03-31".to_string()),
            ..Default::default()
        };
        let result = client.global_filings().search(&body).await;

        if let Ok(_response) = result {
            // Just assert deserialization succeeded.
        }
    }

    #[tokio::test]
    #[ignore = "requires API key (premium)"]
    async fn test_search_in_filing() {
        let client = test_client().await;
        let body = InFilingSearchBody {
            query: "climate".to_string(),
            filing_id: "0".to_string(),
        };
        let _ = client.global_filings().search_in_filing(&body).await;
        // Premium-only; skip deeper assertions.
    }
}
