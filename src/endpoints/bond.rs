//! Bond market endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::bond::{BondPrice, BondProfile, BondTickData, BondYieldCurve},
};

/// Bond-related API endpoints.
pub struct BondEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> BondEndpoints<'a> {
    /// Create a new bond endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get bond profile.
    ///
    /// Returns general information about a bond.
    ///
    /// # Arguments
    /// * `figi` - FIGI identifier (optional)
    /// * `isin` - ISIN identifier (optional)
    /// * `cusip` - CUSIP identifier (optional)
    pub async fn profile(
        &self,
        figi: Option<&str>,
        isin: Option<&str>,
        cusip: Option<&str>,
    ) -> Result<BondProfile> {
        let mut params = vec![];
        
        if let Some(f) = figi {
            params.push(format!("figi={}", f));
        }
        if let Some(i) = isin {
            params.push(format!("isin={}", i));
        }
        if let Some(c) = cusip {
            params.push(format!("cusip={}", c));
        }
        
        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either FIGI, ISIN, or CUSIP must be provided".to_string(),
            ));
        }
        
        let query = format!("/bond/profile?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get bond price.
    ///
    /// Returns current bond price data.
    ///
    /// # Arguments
    /// * `isin` - ISIN identifier
    pub async fn price(&self, isin: &str) -> Result<BondPrice> {
        self.client
            .get(&format!("/bond/price?isin={}", isin))
            .await
    }

    /// Get bond tick data.
    ///
    /// Returns historical tick data for bonds.
    ///
    /// # Arguments
    /// * `isin` - ISIN identifier
    /// * `date` - Date in YYYY-MM-DD format
    /// * `limit` - Limit number of ticks returned (max 25000)
    /// * `skip` - Number of ticks to skip
    /// * `exchange` - Exchange (e.g., "trace")
    pub async fn tick(
        &self,
        isin: &str,
        date: &str,
        limit: i64,
        skip: i64,
        exchange: &str,
    ) -> Result<BondTickData> {
        self.client
            .get(&format!(
                "/bond/tick?isin={}&date={}&limit={}&skip={}&exchange={}",
                isin, date, limit, skip, exchange
            ))
            .await
    }

    /// Get bond yield curve.
    ///
    /// Returns yield curve data for Treasury bonds.
    ///
    /// # Arguments
    /// * `code` - Bond's code (e.g., "10y" for 10-year Treasury)
    pub async fn yield_curve(&self, code: &str) -> Result<BondYieldCurve> {
        self.client
            .get(&format!("/bond/yield-curve?code={}", code))
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
    async fn test_profile() {
        let client = test_client().await;
        let result = client.bond().profile(Some("BBG00B3T3HD3"), None, None).await;
        assert!(result.is_ok(), "Failed to get bond profile: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_price() {
        let client = test_client().await;
        let result = client.bond().price("US037833100").await;
        assert!(result.is_ok(), "Failed to get bond price: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_tick() {
        let client = test_client().await;
        let result = client.bond().tick("2024-01-15", "US037833100", 100, 0, "TRACE").await;
        assert!(result.is_ok(), "Failed to get bond tick: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_yield_curve() {
        let client = test_client().await;
        let result = client.bond().yield_curve("US").await;
        assert!(result.is_ok(), "Failed to get yield curve: {:?}", result.err());
    }
}