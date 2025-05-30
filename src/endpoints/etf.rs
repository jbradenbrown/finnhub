//! ETF (Exchange-Traded Fund) endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::etf::{ETFCountryExposure, ETFHoldings, ETFProfile, ETFSectorExposure},
};

/// ETF-related API endpoints.
pub struct ETFEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> ETFEndpoints<'a> {
    /// Create a new ETF endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get ETF profile.
    ///
    /// Returns comprehensive ETF profile information.
    ///
    /// # Arguments
    /// * `symbol` - ETF symbol (optional if using ISIN)
    /// * `isin` - ETF ISIN (optional if using symbol)
    pub async fn profile(&self, symbol: Option<&str>, isin: Option<&str>) -> Result<ETFProfile> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(i) = isin {
            params.push(format!("isin={}", i));
        }
        
        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either symbol or ISIN must be provided".to_string(),
            ));
        }
        
        let query = format!("/etf/profile?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get ETF holdings/constituents.
    ///
    /// Returns full ETF holdings data.
    ///
    /// # Arguments
    /// * `symbol` - ETF symbol (optional if using ISIN)
    /// * `isin` - ETF ISIN (optional if using symbol)
    /// * `skip` - Skip the first n results (optional)
    /// * `date` - Holdings date in YYYY-MM-DD format (optional)
    pub async fn holdings(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
        skip: Option<i64>,
        date: Option<&str>,
    ) -> Result<ETFHoldings> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(i) = isin {
            params.push(format!("isin={}", i));
        }
        if let Some(sk) = skip {
            params.push(format!("skip={}", sk));
        }
        if let Some(d) = date {
            params.push(format!("date={}", d));
        }
        
        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either symbol or ISIN must be provided".to_string(),
            ));
        }
        
        let query = format!("/etf/holdings?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get ETF country exposure.
    ///
    /// Returns geographical allocation data for the ETF.
    ///
    /// # Arguments
    /// * `symbol` - ETF symbol (optional if using ISIN)
    /// * `isin` - ETF ISIN (optional if using symbol)
    pub async fn country_exposure(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
    ) -> Result<ETFCountryExposure> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(i) = isin {
            params.push(format!("isin={}", i));
        }
        
        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either symbol or ISIN must be provided".to_string(),
            ));
        }
        
        let query = format!("/etf/country?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get ETF sector exposure.
    ///
    /// Returns sector allocation data for the ETF.
    ///
    /// # Arguments
    /// * `symbol` - ETF symbol (optional if using ISIN)
    /// * `isin` - ETF ISIN (optional if using symbol)
    pub async fn sector_exposure(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
    ) -> Result<ETFSectorExposure> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(i) = isin {
            params.push(format!("isin={}", i));
        }
        
        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either symbol or ISIN must be provided".to_string(),
            ));
        }
        
        let query = format!("/etf/sector?{}", params.join("&"));
        self.client.get(&query).await
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
        let result = client.etf().profile(Some("SPY"), None).await;
        assert!(result.is_ok(), "Failed to get ETF profile: {:?}", result.err());
        
        let profile = result.unwrap();
        assert!(profile.profile.name.is_some());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_holdings() {
        let client = test_client().await;
        let result = client.etf().holdings(Some("SPY"), None, None, None).await;
        assert!(result.is_ok(), "Failed to get ETF holdings: {:?}", result.err());
        
        let holdings = result.unwrap();
        assert!(!holdings.holdings.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_country_exposure() {
        let client = test_client().await;
        let result = client.etf().country_exposure(Some("SPY"), None).await;
        assert!(result.is_ok(), "Failed to get country exposure: {:?}", result.err());
        
        let exposure = result.unwrap();
        assert!(!exposure.country_exposure.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sector_exposure() {
        let client = test_client().await;
        let result = client.etf().sector_exposure(Some("SPY"), None).await;
        assert!(result.is_ok(), "Failed to get sector exposure: {:?}", result.err());
        
        let exposure = result.unwrap();
        assert!(!exposure.sector_exposure.is_empty());
    }
}