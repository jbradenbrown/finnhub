//! Mutual fund endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::mutual_fund::{
        MutualFundCountryExposureData, MutualFundEET, MutualFundEETPAI, MutualFundHoldings,
        MutualFundProfile, MutualFundSectorExposureData,
    },
};

/// Mutual fund-related API endpoints.
pub struct MutualFundEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> MutualFundEndpoints<'a> {
    /// Create a new mutual fund endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get mutual fund profile.
    ///
    /// Returns comprehensive mutual fund profile information.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (optional if using ISIN)
    /// * `isin` - Fund ISIN (optional if using symbol)
    pub async fn profile(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
    ) -> Result<MutualFundProfile> {
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

        let query = format!("/mutual-fund/profile?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get mutual fund holdings.
    ///
    /// Returns full mutual fund holdings data.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (optional if using ISIN)
    /// * `isin` - Fund ISIN (optional if using symbol)
    /// * `skip` - Skip the first n results (optional)
    pub async fn holdings(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
        skip: Option<i64>,
    ) -> Result<MutualFundHoldings> {
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

        if params.is_empty() {
            return Err(crate::error::Error::InvalidRequest(
                "Either symbol or ISIN must be provided".to_string(),
            ));
        }

        let query = format!("/mutual-fund/holdings?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get mutual fund country exposure.
    ///
    /// Returns geographical allocation data for the fund.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (optional if using ISIN)
    /// * `isin` - Fund ISIN (optional if using symbol)
    pub async fn country_exposure(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
    ) -> Result<MutualFundCountryExposureData> {
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

        let query = format!("/mutual-fund/country?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get mutual fund sector exposure.
    ///
    /// Returns sector allocation data for the fund.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (optional if using ISIN)
    /// * `isin` - Fund ISIN (optional if using symbol)
    pub async fn sector_exposure(
        &self,
        symbol: Option<&str>,
        isin: Option<&str>,
    ) -> Result<MutualFundSectorExposureData> {
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

        let query = format!("/mutual-fund/sector?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get mutual fund EET (European ESG Template) data.
    ///
    /// Returns ESG-related information required for EU compliance.
    ///
    /// # Arguments
    /// * `isin` - Fund ISIN
    pub async fn eet(&self, isin: &str) -> Result<MutualFundEET> {
        self.client
            .get(&format!("/mutual-fund/eet?isin={}", isin))
            .await
    }

    /// Get mutual fund EET PAI (Principal Adverse Impact) data.
    ///
    /// Returns Principal Adverse Impact indicators for EU funds.
    ///
    /// # Arguments
    /// * `isin` - Fund ISIN
    pub async fn eet_pai(&self, isin: &str) -> Result<MutualFundEETPAI> {
        self.client
            .get(&format!("/mutual-fund/eet-pai?isin={}", isin))
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
    async fn test_profile() {
        let client = test_client().await;
        let result = client.mutual_fund().profile(Some("VTSAX"), None).await;
        assert!(
            result.is_ok(),
            "Failed to get mutual fund profile: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_holdings() {
        let client = test_client().await;
        let result = client
            .mutual_fund()
            .holdings(Some("VTSAX"), None, None)
            .await;
        assert!(
            result.is_ok(),
            "Failed to get mutual fund holdings: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_country_exposure() {
        let client = test_client().await;
        let result = client
            .mutual_fund()
            .country_exposure(Some("VTSAX"), None)
            .await;
        assert!(
            result.is_ok(),
            "Failed to get country exposure: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sector_exposure() {
        let client = test_client().await;
        let result = client
            .mutual_fund()
            .sector_exposure(Some("VTSAX"), None)
            .await;
        assert!(
            result.is_ok(),
            "Failed to get sector exposure: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_eet() {
        let client = test_client().await;
        let result = client.mutual_fund().eet("US922908769").await;
        assert!(result.is_ok(), "Failed to get EET: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_eet_pai() {
        let client = test_client().await;
        let result = client.mutual_fund().eet_pai("US922908769").await;
        assert!(result.is_ok(), "Failed to get EET PAI: {:?}", result.err());
    }
}
