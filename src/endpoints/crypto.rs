//! Cryptocurrency endpoints.

use crate::{client::FinnhubClient, error::Result, models::crypto::*};

/// Crypto-related API endpoints.
pub struct CryptoEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> CryptoEndpoints<'a> {
    /// Create a new crypto endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }
    
    /// Get supported crypto exchanges.
    pub async fn exchanges(&self) -> Result<Vec<CryptoExchange>> {
        self.client.get("/crypto/exchange").await
    }
    
    /// Get supported crypto symbols.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<CryptoSymbol>> {
        self.client
            .get(&format!("/crypto/symbol?exchange={}", exchange))
            .await
    }
}