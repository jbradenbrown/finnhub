//! Forex market endpoints.

use crate::{client::FinnhubClient, error::Result, models::forex::*};

/// Forex-related API endpoints.
pub struct ForexEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> ForexEndpoints<'a> {
    /// Create a new forex endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get supported forex symbols.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<ForexSymbol>> {
        self.client
            .get(&format!("/forex/symbol?exchange={}", exchange))
            .await
    }
}
