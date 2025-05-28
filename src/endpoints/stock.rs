//! Stock market endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        BasicFinancials, CandleResolution, CompanyProfile, Dividend, Earnings, FinancialStatements,
        HistoricalESG, HistoricalEmployeeCount, HistoricalMarketCapData, InsiderSentimentData,
        InsiderTransactions, MarketStatus, OwnershipData, PriceTarget, Quote, RecommendationTrend,
        RevenueBreakdown, StatementFrequency, StatementType, StockCandles, StockSplit, Symbol,
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

    /// Get basic financials metrics.
    ///
    /// Returns key metrics such as P/E ratio, market cap, 52-week high/low, etc.
    pub async fn metrics(&self, symbol: &str) -> Result<BasicFinancials> {
        self.client
            .get(&format!("/stock/metric?symbol={}&metric=all", symbol))
            .await
    }

    /// Get company earnings.
    pub async fn earnings(&self, symbol: &str, limit: Option<i64>) -> Result<Vec<Earnings>> {
        let url = if let Some(limit) = limit {
            format!("/stock/earnings?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/earnings?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get dividends data.
    ///
    /// Returns dividend history with dates and amounts.
    pub async fn dividends(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<Dividend>> {
        self.client
            .get(&format!(
                "/stock/dividend?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get stock splits history.
    ///
    /// Returns stock split history with dates and split ratios.
    pub async fn splits(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<StockSplit>> {
        self.client
            .get(&format!(
                "/stock/split?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get list of supported stocks.
    ///
    /// List all supported stocks for a given exchange.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<Symbol>> {
        self.client
            .get(&format!("/stock/symbol?exchange={}", exchange))
            .await
    }

    /// Get historical market capitalization data.
    ///
    /// Returns historical market cap values for a given date range.
    pub async fn historical_market_cap(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalMarketCapData> {
        self.client
            .get(&format!(
                "/stock/historical-market-cap?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get historical employee count data.
    ///
    /// Returns historical employee count for a given date range.
    pub async fn historical_employee_count(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalEmployeeCount> {
        self.client
            .get(&format!(
                "/stock/historical-employee-count?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get historical ESG (Environmental, Social, Governance) scores.
    ///
    /// Returns historical ESG scores for a given date range.
    pub async fn historical_esg(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalESG> {
        self.client
            .get(&format!(
                "/stock/historical-esg?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get company peers.
    ///
    /// Returns a list of peers operating in the same country and sector/industry.
    pub async fn peers(&self, symbol: &str, grouping: Option<&str>) -> Result<Vec<String>> {
        let url = if let Some(grouping) = grouping {
            format!("/stock/peers?symbol={}&grouping={}", symbol, grouping)
        } else {
            format!("/stock/peers?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get current market status.
    ///
    /// Returns whether the exchange is open or closed.
    pub async fn market_status(&self, exchange: &str) -> Result<MarketStatus> {
        self.client
            .get(&format!("/stock/market-status?exchange={}", exchange))
            .await
    }

    /// Get company ownership data.
    ///
    /// Returns a list of company shareholders/owners.
    pub async fn ownership(&self, symbol: &str, limit: Option<i64>) -> Result<OwnershipData> {
        let url = if let Some(limit) = limit {
            format!("/stock/ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get revenue breakdown data.
    ///
    /// Returns revenue breakdown by business segment, product, or geography.
    pub async fn revenue_breakdown(&self, symbol: &str) -> Result<RevenueBreakdown> {
        self.client
            .get(&format!("/stock/revenue-breakdown?symbol={}", symbol))
            .await
    }

    /// Get insider sentiment data.
    ///
    /// Returns aggregated insider trading sentiment by month.
    pub async fn insider_sentiment(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<InsiderSentimentData> {
        self.client
            .get(&format!(
                "/stock/insider-sentiment?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }
}
