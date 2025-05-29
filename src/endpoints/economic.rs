//! Economic data endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::economic::{EconomicCode, EconomicData},
};

/// Economic data API endpoints.
pub struct EconomicEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> EconomicEndpoints<'a> {
    /// Create a new economic endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get economic data.
    ///
    /// Returns economic data for a specific indicator.
    ///
    /// # Arguments
    /// * `code` - Economic indicator code (e.g., "MA-USA-656880")
    pub async fn data(&self, code: &str) -> Result<EconomicData> {
        self.client
            .get(&format!("/economic?code={}", code))
            .await
    }

    /// Get list of economic indicator codes.
    ///
    /// Returns all available economic indicator codes.
    pub async fn codes(&self) -> Result<Vec<EconomicCode>> {
        self.client.get("/economic/code").await
    }
}