//! Miscellaneous API endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::misc::{
        AIChatRequest, AIChatResponse, AirlinePriceIndexData, CountryMetadata, CovidInfo,
        FDACommitteeMeeting, PressRelease, SectorMetric, SymbolLookup, TechnicalIndicator,
    },
};

/// Miscellaneous API endpoints.
pub struct MiscEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> MiscEndpoints<'a> {
    /// Create a new instance of misc endpoints.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Chat with AI copilot powered by Neyman AI.
    pub async fn ai_chat(&self, _request: &AIChatRequest) -> Result<AIChatResponse> {
        // Note: This is a POST endpoint, which would require implementing POST support in the client
        // For now, this is a placeholder
        unimplemented!("POST endpoints not yet implemented")
    }

    /// Get airline ticket price index.
    pub async fn airline_price_index(
        &self,
        airline: &str,
        from: &str,
        to: &str,
    ) -> Result<AirlinePriceIndexData> {
        self.client
            .get(&format!(
                "/airline/price-index?airline={}&from={}&to={}",
                airline, from, to
            ))
            .await
    }

    /// List all countries and metadata.
    pub async fn country(&self) -> Result<Vec<CountryMetadata>> {
        self.client.get("/country").await
    }

    /// Get real-time COVID-19 data for US states.
    pub async fn covid19(&self) -> Result<Vec<CovidInfo>> {
        self.client.get("/covid19/us").await
    }

    /// FDA's advisory committee calendar.
    pub async fn fda_calendar(&self) -> Result<Vec<FDACommitteeMeeting>> {
        self.client.get("/fda-advisory-committee-calendar").await
    }

    /// Get technical indicator with price data.
    pub async fn technical_indicator(
        &self,
        symbol: &str,
        resolution: &str,
        from: i64,
        to: i64,
        indicator: &str,
        indicator_fields: Option<serde_json::Value>,
    ) -> Result<TechnicalIndicator> {
        let mut url = format!(
            "/indicator?symbol={}&resolution={}&from={}&to={}&indicator={}",
            symbol, resolution, from, to, indicator
        );

        if let Some(fields) = indicator_fields {
            // Add indicator fields as query parameters
            if let Some(obj) = fields.as_object() {
                for (key, value) in obj {
                    url.push_str(&format!("&{}={}", key, value));
                }
            }
        }

        self.client.get(&url).await
    }

    /// Get latest major press releases of a company.
    pub async fn press_releases(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<PressRelease> {
        let mut url = format!("/press-releases?symbol={}", symbol);

        if let Some(from_date) = from {
            url.push_str(&format!("&from={}", from_date));
        }

        if let Some(to_date) = to {
            url.push_str(&format!("&to={}", to_date));
        }

        self.client.get(&url).await
    }

    /// Search for best-matching symbols based on query.
    pub async fn symbol_search(&self, query: &str, exchange: Option<&str>) -> Result<SymbolLookup> {
        let mut url = format!("/search?q={}", query);

        if let Some(ex) = exchange {
            url.push_str(&format!("&exchange={}", ex));
        }

        self.client.get(&url).await
    }

    /// Get ratios for different sectors and regions/indices.
    pub async fn sector_metrics(&self, region: &str) -> Result<SectorMetric> {
        self.client
            .get(&format!("/sector/metrics?region={}", region))
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
    async fn test_airline_price_index() {
        let client = test_client().await;
        let result = client.misc().airline_price_index("LUV", "2024-01-01", "2024-01-31").await;
        assert!(result.is_ok(), "Failed to get airline price index: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_country() {
        let client = test_client().await;
        let result = client.misc().country().await;
        assert!(result.is_ok(), "Failed to get country data: {:?}", result.err());
        
        let countries = result.unwrap();
        assert!(!countries.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_covid19() {
        let client = test_client().await;
        let result = client.misc().covid19().await;
        assert!(result.is_ok(), "Failed to get COVID-19 data: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_fda_calendar() {
        let client = test_client().await;
        let result = client.misc().fda_calendar().await;
        assert!(result.is_ok(), "Failed to get FDA calendar: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_technical_indicator() {
        let client = test_client().await;
        let from = chrono::Utc::now().timestamp() - 86400 * 30;
        let to = chrono::Utc::now().timestamp();
        let mut params = serde_json::Map::new();
        params.insert("timeperiod".to_string(), serde_json::json!(14));
        let result = client.misc().technical_indicator("AAPL", "D", from, to, "sma", Some(serde_json::Value::Object(params))).await;
        assert!(result.is_ok(), "Failed to get technical indicator: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_press_releases() {
        let client = test_client().await;
        let from = "2024-01-01";
        let to = "2024-01-31";
        let result = client.misc().press_releases("AAPL", Some(from), Some(to)).await;
        assert!(result.is_ok(), "Failed to get press releases: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_symbol_search() {
        let client = test_client().await;
        let result = client.misc().symbol_search("apple", None).await;
        assert!(result.is_ok(), "Failed to search symbols: {:?}", result.err());
        
        let results = result.unwrap();
        assert!(!results.result.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sector_metrics() {
        let client = test_client().await;
        let result = client.misc().sector_metrics("US").await;
        assert!(result.is_ok(), "Failed to get sector metrics: {:?}", result.err());
    }
}