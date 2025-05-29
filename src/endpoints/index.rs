//! Index-related endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::index::{IndicesConstituents, IndicesHistoricalConstituents},
};

/// Index-related API endpoints.
pub struct IndexEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> IndexEndpoints<'a> {
    /// Create a new index endpoints instance.
    #[must_use]
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get index constituents.
    ///
    /// Get a list of index's constituents.
    ///
    /// # Arguments
    /// * `symbol` - Index symbol (e.g., "^GSPC" for S&P 500, "^NDX" for Nasdaq 100)
    ///
    /// # Errors
    /// Returns an error if the API request fails.
    pub async fn constituents(&self, symbol: &str) -> Result<IndicesConstituents> {
        self.client
            .get(&format!("/index/constituents?symbol={}", symbol))
            .await
    }

    /// Get historical index constituents.
    ///
    /// Get full history of index's constituents including symbols and dates of joining and leaving.
    /// Currently supports ^GSPC, ^NDX, ^DJI.
    ///
    /// # Arguments
    /// * `symbol` - Index symbol
    ///
    /// # Errors
    /// Returns an error if the API request fails.
    pub async fn historical_constituents(&self, symbol: &str) -> Result<IndicesHistoricalConstituents> {
        self.client
            .get(&format!("/index/historical-constituents?symbol={}", symbol))
            .await
    }
}