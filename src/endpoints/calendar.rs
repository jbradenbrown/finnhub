//! Calendar endpoints for earnings, economic events, and IPOs.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::{
        calendar::{EarningsCalendar, EconomicCalendar},
        stock::corporate_actions::IPOCalendar,
    },
};

/// Calendar-related API endpoints.
pub struct CalendarEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> CalendarEndpoints<'a> {
    /// Create a new calendar endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get earnings calendar.
    ///
    /// Returns historical and upcoming earnings releases.
    ///
    /// # Arguments
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    /// * `symbol` - Filter by symbol (optional)
    pub async fn earnings(
        &self,
        from: Option<&str>,
        to: Option<&str>,
        symbol: Option<&str>,
    ) -> Result<EarningsCalendar> {
        let mut params = vec![];
        
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        
        let query = if params.is_empty() {
            String::from("/calendar/earnings")
        } else {
            format!("/calendar/earnings?{}", params.join("&"))
        };
        
        self.client.get(&query).await
    }

    /// Get economic calendar.
    ///
    /// Returns recent and upcoming economic releases.
    ///
    /// # Arguments
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    pub async fn economic(&self, from: Option<&str>, to: Option<&str>) -> Result<EconomicCalendar> {
        let mut params = vec![];
        
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        
        let query = if params.is_empty() {
            String::from("/calendar/economic")
        } else {
            format!("/calendar/economic?{}", params.join("&"))
        };
        
        self.client.get(&query).await
    }

    /// Get IPO calendar.
    ///
    /// Returns recent and upcoming IPOs for a date range.
    ///
    /// # Arguments
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn ipo(&self, from: &str, to: &str) -> Result<IPOCalendar> {
        self.client
            .get(&format!("/calendar/ipo?from={}&to={}", from, to))
            .await
    }
}