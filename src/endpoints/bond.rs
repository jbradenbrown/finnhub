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