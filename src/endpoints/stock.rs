//! Stock market endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        CandleResolution, CompanyProfile, FinancialStatements, InsiderTransactions, PriceTarget,
        Quote, RecommendationTrend, StatementFrequency, StatementType, StockCandles,
    },
};

/// Stock-related API endpoints.
pub struct StockEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> StockEndpoints<'a> {
    /// Create a new stock endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get real-time quote data.
    pub async fn quote(&self, symbol: &str) -> Result<Quote> {
        self.client.get(&format!("/quote?symbol={}", symbol)).await
    }

    /// Get company profile.
    pub async fn company_profile(&self, symbol: &str) -> Result<CompanyProfile> {
        self.client
            .get(&format!("/stock/profile2?symbol={}", symbol))
            .await
    }

    /// Get candlestick data (OHLCV) for stocks.
    ///
    /// Daily data will be adjusted for splits. Intraday data will remain unadjusted.
    /// Only 1 month of intraday data will be returned at a time.
    pub async fn candles(
        &self,
        symbol: &str,
        resolution: CandleResolution,
        from: i64,
        to: i64,
    ) -> Result<StockCandles> {
        self.client
            .get(&format!(
                "/stock/candle?symbol={}&resolution={}&from={}&to={}",
                symbol, resolution, from, to
            ))
            .await
    }

    /// Get standardized financial statements.
    ///
    /// Get balance sheet, income statement, or cash flow for global companies.
    pub async fn financials(
        &self,
        symbol: &str,
        statement: StatementType,
        frequency: StatementFrequency,
    ) -> Result<FinancialStatements> {
        self.client
            .get(&format!(
                "/stock/financials?symbol={}&statement={}&freq={}",
                symbol, statement, frequency
            ))
            .await
    }

    /// Get latest price target consensus.
    pub async fn price_target(&self, symbol: &str) -> Result<PriceTarget> {
        self.client
            .get(&format!("/stock/price-target?symbol={}", symbol))
            .await
    }

    /// Get latest analyst recommendations.
    pub async fn recommendations(&self, symbol: &str) -> Result<Vec<RecommendationTrend>> {
        self.client
            .get(&format!("/stock/recommendation?symbol={}", symbol))
            .await
    }

    /// Get insider transactions.
    ///
    /// Returns insider transactions for the last 3 months.
    pub async fn insider_transactions(&self, symbol: &str) -> Result<InsiderTransactions> {
        self.client
            .get(&format!("/stock/insider-transactions?symbol={}", symbol))
            .await
    }
}
