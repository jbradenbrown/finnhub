//! Stock market endpoints organized by category.

pub mod analytics;
pub mod company;
pub mod compliance;
pub mod corporate_actions;
pub mod estimates;
pub mod filings;
pub mod financials;
pub mod historical;
pub mod insider;
pub mod market;
pub mod ownership;
pub mod price;
pub mod sentiment;

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::*,
};

/// Stock-related API endpoints with a flat API structure.
pub struct StockEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> StockEndpoints<'a> {
    /// Create a new stock endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    // ===== Price endpoints =====
    
    /// Get real-time quote data.
    pub async fn quote(&self, symbol: &str) -> Result<Quote> {
        price::PriceEndpoints::new(self.client).quote(symbol).await
    }

    /// Get candlestick data (OHLCV) for stocks.
    pub async fn candles(
        &self,
        symbol: &str,
        resolution: CandleResolution,
        from: i64,
        to: i64,
    ) -> Result<StockCandles> {
        price::PriceEndpoints::new(self.client).candles(symbol, resolution, from, to).await
    }

    /// Get last bid-ask data.
    pub async fn bid_ask(&self, symbol: &str) -> Result<BidAsk> {
        price::PriceEndpoints::new(self.client).bid_ask(symbol).await
    }

    /// Get historical tick data.
    pub async fn tick_data(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<TickData> {
        price::PriceEndpoints::new(self.client).tick_data(symbol, date, limit, skip).await
    }

    /// Get price metrics.
    pub async fn price_metrics(&self, symbol: &str) -> Result<PriceMetrics> {
        price::PriceEndpoints::new(self.client).price_metrics(symbol).await
    }

    // ===== Company endpoints =====
    
    /// Get company profile.
    pub async fn company_profile(&self, symbol: &str) -> Result<CompanyProfile> {
        company::CompanyEndpoints::new(self.client).profile(symbol).await
    }

    /// Get company peers.
    pub async fn peers(&self, symbol: &str, grouping: Option<&str>) -> Result<Vec<String>> {
        company::CompanyEndpoints::new(self.client).peers(symbol, grouping).await
    }

    /// Get list of supported stocks.
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<Symbol>> {
        company::CompanyEndpoints::new(self.client).symbols(exchange).await
    }

    // ===== Financial endpoints =====
    
    /// Get standardized financial statements.
    pub async fn financials(
        &self,
        symbol: &str,
        statement: StatementType,
        frequency: StatementFrequency,
    ) -> Result<FinancialStatements> {
        financials::FinancialsEndpoints::new(self.client).statements(symbol, statement, frequency).await
    }

    /// Get basic financials metrics.
    pub async fn metrics(&self, symbol: &str) -> Result<BasicFinancials> {
        financials::FinancialsEndpoints::new(self.client).metrics(symbol).await
    }

    /// Get company earnings.
    pub async fn earnings(&self, symbol: &str, limit: Option<i64>) -> Result<Vec<Earnings>> {
        financials::FinancialsEndpoints::new(self.client).earnings(symbol, limit).await
    }

    /// Get financials as reported.
    pub async fn financials_reported(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        freq: Option<&str>,
    ) -> Result<FinancialsAsReported> {
        financials::FinancialsEndpoints::new(self.client).as_reported(symbol, cik, access_number, freq).await
    }

    // ===== Analytics endpoints =====
    
    /// Get latest price target consensus.
    pub async fn price_target(&self, symbol: &str) -> Result<PriceTarget> {
        analytics::AnalyticsEndpoints::new(self.client).price_target(symbol).await
    }

    /// Get latest analyst recommendations.
    pub async fn recommendations(&self, symbol: &str) -> Result<Vec<RecommendationTrend>> {
        analytics::AnalyticsEndpoints::new(self.client).recommendations(symbol).await
    }

    /// Get revenue breakdown data.
    pub async fn revenue_breakdown(&self, symbol: &str) -> Result<RevenueBreakdown> {
        analytics::AnalyticsEndpoints::new(self.client).revenue_breakdown(symbol).await
    }

    /// Get stock upgrades and downgrades.
    pub async fn upgrade_downgrade(
        &self,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<UpgradeDowngrade>> {
        analytics::AnalyticsEndpoints::new(self.client).upgrade_downgrade(symbol, from, to).await
    }

    // ===== Insider endpoints =====
    
    /// Get insider transactions.
    pub async fn insider_transactions(&self, symbol: &str) -> Result<InsiderTransactions> {
        insider::InsiderEndpoints::new(self.client).transactions(symbol).await
    }

    /// Get insider sentiment data.
    pub async fn insider_sentiment(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<InsiderSentimentData> {
        insider::InsiderEndpoints::new(self.client).sentiment(symbol, from, to).await
    }

    // ===== Corporate actions endpoints =====
    
    /// Get dividends data.
    pub async fn dividends(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<Dividend>> {
        corporate_actions::CorporateActionsEndpoints::new(self.client).dividends(symbol, from, to).await
    }

    /// Get stock splits history.
    pub async fn splits(&self, symbol: &str, from: &str, to: &str) -> Result<Vec<StockSplit>> {
        corporate_actions::CorporateActionsEndpoints::new(self.client).splits(symbol, from, to).await
    }

    /// Get dividends v2.
    pub async fn dividends_v2(&self, symbol: &str) -> Result<DividendsV2> {
        corporate_actions::CorporateActionsEndpoints::new(self.client).dividends_v2(symbol).await
    }

    // ===== Historical endpoints =====
    
    /// Get historical market capitalization data.
    pub async fn historical_market_cap(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalMarketCapData> {
        historical::HistoricalEndpoints::new(self.client).market_cap(symbol, from, to).await
    }

    /// Get historical employee count data.
    pub async fn historical_employee_count(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalEmployeeCount> {
        historical::HistoricalEndpoints::new(self.client).employee_count(symbol, from, to).await
    }

    /// Get historical ESG scores.
    pub async fn historical_esg(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalESG> {
        historical::HistoricalEndpoints::new(self.client).esg(symbol, from, to).await
    }

    /// Get historical NBBO data.
    pub async fn historical_nbbo(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<HistoricalNBBO> {
        historical::HistoricalEndpoints::new(self.client).nbbo(symbol, date, limit, skip).await
    }

    // ===== Sentiment endpoints =====
    
    /// Get social sentiment data.
    pub async fn social_sentiment(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<SocialSentiment> {
        sentiment::SentimentEndpoints::new(self.client).social(symbol, from, to).await
    }

    /// Get filing sentiment analysis.
    pub async fn filing_sentiment(&self, access_number: &str) -> Result<FilingSentiment> {
        sentiment::SentimentEndpoints::new(self.client).filing(access_number).await
    }

    // ===== Market endpoints =====
    
    /// Get current market status.
    pub async fn market_status(&self, exchange: &str) -> Result<MarketStatus> {
        market::MarketEndpoints::new(self.client).status(exchange).await
    }

    /// Get market holidays.
    pub async fn market_holiday(&self, exchange: &str) -> Result<MarketHoliday> {
        market::MarketEndpoints::new(self.client).holiday(exchange).await
    }

    /// Get investment theme portfolio.
    pub async fn investment_theme(&self, theme: &str) -> Result<InvestmentTheme> {
        market::MarketEndpoints::new(self.client).investment_theme(theme).await
    }

    // ===== Ownership endpoints =====
    
    /// Get company ownership data.
    pub async fn ownership(&self, symbol: &str, limit: Option<i64>) -> Result<OwnershipData> {
        ownership::OwnershipEndpoints::new(self.client).institutional(symbol, limit).await
    }

    /// Get fund ownership.
    pub async fn fund_ownership(&self, symbol: &str, limit: Option<i64>) -> Result<FundOwnership> {
        ownership::OwnershipEndpoints::new(self.client).fund(symbol, limit).await
    }

    // ===== Filings endpoints =====
    
    /// Get SEC filings.
    pub async fn sec_filings(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        form: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<Filing>> {
        filings::FilingsEndpoints::new(self.client).sec(symbol, cik, access_number, form, from, to).await
    }

    /// Get international filings.
    pub async fn international_filings(
        &self,
        symbol: Option<&str>,
        country: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<InternationalFiling>> {
        filings::FilingsEndpoints::new(self.client).international(symbol, country, from, to).await
    }

    /// Get earnings call transcripts.
    pub async fn transcripts(&self, id: &str) -> Result<EarningsCallTranscript> {
        filings::FilingsEndpoints::new(self.client).transcript(id).await
    }

    /// Get earnings call transcripts list.
    pub async fn transcripts_list(&self, symbol: &str) -> Result<EarningsCallTranscriptsList> {
        filings::FilingsEndpoints::new(self.client).transcripts_list(symbol).await
    }

    /// Get earnings call live events.
    pub async fn earnings_call_live(&self, from: &str, to: &str) -> Result<EarningsCallLive> {
        filings::FilingsEndpoints::new(self.client).earnings_call_live(from, to).await
    }

    /// Get investor presentations.
    pub async fn presentations(&self, symbol: &str) -> Result<InvestorPresentations> {
        filings::FilingsEndpoints::new(self.client).presentations(symbol).await
    }

    /// Get document similarity index.
    pub async fn similarity_index(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        freq: Option<&str>,
    ) -> Result<SimilarityIndex> {
        filings::FilingsEndpoints::new(self.client).similarity_index(symbol, cik, freq).await
    }

    // ===== Estimates endpoints =====
    
    /// Get EPS estimates.
    pub async fn eps_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EPSEstimates> {
        estimates::EstimatesEndpoints::new(self.client).eps(symbol, freq).await
    }

    /// Get revenue estimates.
    pub async fn revenue_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<RevenueEstimates> {
        estimates::EstimatesEndpoints::new(self.client).revenue(symbol, freq).await
    }

    /// Get EBITDA estimates.
    pub async fn ebitda_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EBITDAEstimates> {
        estimates::EstimatesEndpoints::new(self.client).ebitda(symbol, freq).await
    }

    /// Get EBIT estimates.
    pub async fn ebit_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EBITEstimates> {
        estimates::EstimatesEndpoints::new(self.client).ebit(symbol, freq).await
    }

    /// Get earnings quality score.
    pub async fn earnings_quality_score(&self, symbol: &str, freq: &str) -> Result<EarningsQualityScore> {
        estimates::EstimatesEndpoints::new(self.client).earnings_quality_score(symbol, freq).await
    }

    // ===== Compliance endpoints =====
    
    /// Get company executives.
    pub async fn executives(&self, symbol: &str) -> Result<CompanyExecutives> {
        compliance::ComplianceEndpoints::new(self.client).executives(symbol).await
    }

    /// Get congressional trading data.
    pub async fn congressional_trading(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<CongressionalTrading> {
        compliance::ComplianceEndpoints::new(self.client).congressional_trading(symbol, from, to).await
    }

    /// Get lobbying data.
    pub async fn lobbying(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Lobbying> {
        compliance::ComplianceEndpoints::new(self.client).lobbying(symbol, from, to).await
    }

    /// Get USA spending data.
    pub async fn usa_spending(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<USASpending> {
        compliance::ComplianceEndpoints::new(self.client).usa_spending(symbol, from, to).await
    }

    /// Get current ESG scores.
    pub async fn esg(&self, symbol: &str) -> Result<ESGScore> {
        compliance::ComplianceEndpoints::new(self.client).esg(symbol).await
    }

    /// Get supply chain relationships.
    pub async fn supply_chain(&self, symbol: &str) -> Result<SupplyChainData> {
        compliance::ComplianceEndpoints::new(self.client).supply_chain(symbol).await
    }

    /// Get USPTO patent applications.
    pub async fn uspto_patents(&self, symbol: &str, from: &str, to: &str) -> Result<USPTOPatents> {
        compliance::ComplianceEndpoints::new(self.client).uspto_patents(symbol, from, to).await
    }

    /// Get visa applications.
    pub async fn visa_applications(&self, symbol: &str, from: &str, to: &str) -> Result<VisaApplications> {
        compliance::ComplianceEndpoints::new(self.client).visa_applications(symbol, from, to).await
    }
}