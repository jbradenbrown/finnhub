//! Stock market endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        BasicFinancials, BidAsk, CandleResolution, CompanyExecutives, CompanyProfile,
        CongressionalTrading, Dividend, DividendsV2, Earnings, EarningsCallLive,
        EarningsCallTranscript, EarningsCallTranscriptsList, EarningsQualityScore, EBITDAEstimates,
        EBITEstimates, EPSEstimates, ESGScore, Filing, FilingSentiment, FinancialStatements,
        FinancialsAsReported, FundOwnership, HistoricalESG, HistoricalEmployeeCount,
        HistoricalMarketCapData, HistoricalNBBO, InsiderSentimentData, InsiderTransactions,
        InternationalFiling, InvestmentTheme, InvestorPresentations, Lobbying, MarketHoliday,
        MarketStatus, OwnershipData, PriceMetrics, PriceTarget, Quote, RecommendationTrend,
        RevenueBreakdown, RevenueEstimates, SimilarityIndex, SocialSentiment, StatementFrequency,
        StatementType, StockCandles, StockSplit, SupplyChainData, Symbol, TickData,
        UpgradeDowngrade, USASpending, USPTOPatents, VisaApplications,
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

    /// Get stock upgrades and downgrades.
    ///
    /// Returns analyst upgrades and downgrades for a company.
    pub async fn upgrade_downgrade(
        &self,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<UpgradeDowngrade>> {
        let mut params = Vec::new();
        if let Some(symbol) = symbol {
            params.push(format!("symbol={}", symbol));
        }
        if let Some(from) = from {
            params.push(format!("from={}", from));
        }
        if let Some(to) = to {
            params.push(format!("to={}", to));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };

        self.client
            .get(&format!("/stock/upgrade-downgrade{}", query))
            .await
    }

    /// Get social sentiment data.
    ///
    /// Returns social media sentiment data for a company.
    pub async fn social_sentiment(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<SocialSentiment> {
        self.client
            .get(&format!(
                "/stock/social-sentiment?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get supply chain relationships.
    ///
    /// Returns companies in the supply chain (suppliers and customers).
    pub async fn supply_chain(&self, symbol: &str) -> Result<SupplyChainData> {
        self.client
            .get(&format!("/stock/supply-chain?symbol={}", symbol))
            .await
    }

    /// Get SEC filings.
    ///
    /// List company's SEC filings. You can filter by symbol, CIK, access number, form type, and date range.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `cik` - CIK number (optional)
    /// * `access_number` - Access number of a specific report (optional)
    /// * `form` - Filter by form type (optional)
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    pub async fn sec_filings(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        form: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<Filing>> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = cik {
            params.push(format!("cik={}", c));
        }
        if let Some(a) = access_number {
            params.push(format!("accessNumber={}", a));
        }
        if let Some(f) = form {
            params.push(format!("form={}", f));
        }
        if let Some(from_date) = from {
            params.push(format!("from={}", from_date));
        }
        if let Some(to_date) = to {
            params.push(format!("to={}", to_date));
        }
        
        let query = if params.is_empty() {
            String::from("/stock/filings")
        } else {
            format!("/stock/filings?{}", params.join("&"))
        };
        
        self.client.get(&query).await
    }

    /// Get last bid-ask data.
    ///
    /// Returns the last bid and ask prices with volumes for US stocks.
    pub async fn bid_ask(&self, symbol: &str) -> Result<BidAsk> {
        self.client
            .get(&format!("/stock/bidask?symbol={}", symbol))
            .await
    }

    /// Get historical tick data.
    ///
    /// Returns historical tick data for global exchanges.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `date` - Date in YYYY-MM-DD format
    /// * `limit` - Limit number of ticks returned (max 25000)
    /// * `skip` - Number of ticks to skip
    pub async fn tick_data(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<TickData> {
        self.client
            .get(&format!(
                "/stock/tick?symbol={}&date={}&limit={}&skip={}",
                symbol, date, limit, skip
            ))
            .await
    }

    /// Get financials as reported.
    ///
    /// Returns detailed financial statements as reported to the SEC.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional if using CIK or access number)
    /// * `cik` - CIK number (optional)
    /// * `access_number` - Access number of a specific report (optional)
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn financials_reported(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        access_number: Option<&str>,
        freq: Option<&str>,
    ) -> Result<FinancialsAsReported> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = cik {
            params.push(format!("cik={}", c));
        }
        if let Some(a) = access_number {
            params.push(format!("accessNumber={}", a));
        }
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        let query = if params.is_empty() {
            String::from("/stock/financials-reported")
        } else {
            format!("/stock/financials-reported?{}", params.join("&"))
        };
        
        self.client.get(&query).await
    }

    /// Get company executives.
    ///
    /// Returns a list of company's executives and board members with compensation data.
    pub async fn executives(&self, symbol: &str) -> Result<CompanyExecutives> {
        self.client
            .get(&format!("/stock/executive?symbol={}", symbol))
            .await
    }

    /// Get congressional trading data.
    ///
    /// Returns trading activity by US congress members for a symbol.
    pub async fn congressional_trading(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<CongressionalTrading> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        
        let query = format!("/stock/congressional-trading?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get lobbying data.
    ///
    /// Returns lobbying activities for a company.
    pub async fn lobbying(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Lobbying> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        
        let query = format!("/stock/lobbying?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get USA spending data.
    ///
    /// Returns government contracts awarded to a company.
    pub async fn usa_spending(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<USASpending> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        
        let query = format!("/stock/usa-spending?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get EPS estimates.
    ///
    /// Returns analysts' EPS estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn eps_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EPSEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        let query = format!("/stock/eps-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get revenue estimates.
    ///
    /// Returns analysts' revenue estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn revenue_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<RevenueEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        let query = format!("/stock/revenue-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get EBITDA estimates.
    ///
    /// Returns analysts' EBITDA estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn ebitda_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EBITDAEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        let query = format!("/stock/ebitda-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get EBIT estimates.
    ///
    /// Returns analysts' EBIT estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn ebit_estimates(&self, symbol: &str, freq: Option<&str>) -> Result<EBITEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];
        
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        let query = format!("/stock/ebit-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get earnings quality score.
    ///
    /// Returns earnings quality indicators for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly
    pub async fn earnings_quality_score(&self, symbol: &str, freq: &str) -> Result<Vec<EarningsQualityScore>> {
        self.client
            .get(&format!("/stock/earnings-quality-score?symbol={}&freq={}", symbol, freq))
            .await
    }

    /// Get historical NBBO (National Best Bid and Offer) data.
    ///
    /// Returns historical best bid and offer for US stocks, LSE, TSX, Euronext and Deutsche Borse.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `date` - Date in YYYY-MM-DD format
    /// * `limit` - Limit number of ticks returned (max 25000)
    /// * `skip` - Number of ticks to skip
    pub async fn historical_nbbo(
        &self,
        symbol: &str,
        date: &str,
        limit: i64,
        skip: i64,
    ) -> Result<HistoricalNBBO> {
        self.client
            .get(&format!(
                "/stock/bbo?symbol={}&date={}&limit={}&skip={}",
                symbol, date, limit, skip
            ))
            .await
    }

    /// Get investment theme portfolio.
    ///
    /// Returns portfolios of different investment themes that are changing our life and are the way of the future.
    ///
    /// # Arguments
    /// * `theme` - Investment theme (e.g., "financialExchangesData", "futureFood")
    pub async fn investment_theme(&self, theme: &str) -> Result<InvestmentTheme> {
        self.client
            .get(&format!("/stock/investment-theme?theme={}", theme))
            .await
    }

    /// Get market holidays.
    ///
    /// Returns a list of holidays for global exchanges.
    ///
    /// # Arguments
    /// * `exchange` - Exchange code
    pub async fn market_holiday(&self, exchange: &str) -> Result<MarketHoliday> {
        self.client
            .get(&format!("/stock/market-holiday?exchange={}", exchange))
            .await
    }

    /// Get international filings.
    ///
    /// List filings for international companies. Limit to 500 documents at a time.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `country` - Filter by country using country's 2-letter code (optional)
    /// * `from` - From date in YYYY-MM-DD format (optional)
    /// * `to` - To date in YYYY-MM-DD format (optional)
    pub async fn international_filings(
        &self,
        symbol: Option<&str>,
        country: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<InternationalFiling>> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = country {
            params.push(format!("country={}", c));
        }
        if let Some(f) = from {
            params.push(format!("from={}", f));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t));
        }
        
        let query = if params.is_empty() {
            String::from("/stock/international-filings")
        } else {
            format!("/stock/international-filings?{}", params.join("&"))
        };
        
        self.client.get(&query).await
    }

    /// Get earnings call transcripts.
    ///
    /// Get earnings call transcripts, audio and participants' list.
    ///
    /// # Arguments
    /// * `id` - Transcript's id obtained with Transcripts List endpoint
    pub async fn transcripts(&self, id: &str) -> Result<EarningsCallTranscript> {
        self.client
            .get(&format!("/stock/transcripts?id={}", id))
            .await
    }

    /// Get earnings call transcripts list.
    ///
    /// List earnings call transcripts' metadata.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn transcripts_list(&self, symbol: &str) -> Result<EarningsCallTranscriptsList> {
        self.client
            .get(&format!("/stock/transcripts/list?symbol={}", symbol))
            .await
    }

    /// Get current ESG scores.
    ///
    /// Get current ESG (Environmental, Social, Governance) scores for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn esg(&self, symbol: &str) -> Result<ESGScore> {
        self.client
            .get(&format!("/stock/esg?symbol={}", symbol))
            .await
    }

    /// Get fund ownership.
    ///
    /// Get a list of funds that hold shares of a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `limit` - Limit number of results (optional)
    pub async fn fund_ownership(&self, symbol: &str, limit: Option<i64>) -> Result<FundOwnership> {
        let url = if let Some(limit) = limit {
            format!("/stock/fund-ownership?symbol={}&limit={}", symbol, limit)
        } else {
            format!("/stock/fund-ownership?symbol={}", symbol)
        };
        self.client.get(&url).await
    }

    /// Get USPTO patent applications.
    ///
    /// List USPTO patent applications for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn uspto_patents(&self, symbol: &str, from: &str, to: &str) -> Result<USPTOPatents> {
        self.client
            .get(&format!(
                "/stock/uspto-patent?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get visa applications.
    ///
    /// List H1B visa applications for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn visa_applications(&self, symbol: &str, from: &str, to: &str) -> Result<VisaApplications> {
        self.client
            .get(&format!(
                "/stock/visa-application?symbol={}&from={}&to={}",
                symbol, from, to
            ))
            .await
    }

    /// Get filing sentiment analysis.
    ///
    /// Analyze sentiment of company filings using NLP.
    ///
    /// # Arguments
    /// * `access_number` - Access number of the filing
    pub async fn filing_sentiment(&self, access_number: &str) -> Result<FilingSentiment> {
        self.client
            .get(&format!("/stock/filings-sentiment?accessNumber={}", access_number))
            .await
    }

    /// Get document similarity index.
    ///
    /// Compare company filings and estimate percentage of changes.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    /// * `cik` - CIK number (optional)
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn similarity_index(
        &self,
        symbol: Option<&str>,
        cik: Option<&str>,
        freq: Option<&str>,
    ) -> Result<SimilarityIndex> {
        let mut params = vec![];
        
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        if let Some(c) = cik {
            params.push(format!("cik={}", c));
        }
        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }
        
        if params.is_empty() {
            return Err(crate::Error::InvalidRequest(
                "At least one of symbol or cik must be provided".to_string()
            ));
        }
        
        let query = format!("/stock/similarity-index?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get earnings call live events.
    ///
    /// Get upcoming earnings call events that support live audio streaming.
    ///
    /// # Arguments
    /// * `from` - From date in YYYY-MM-DD format
    /// * `to` - To date in YYYY-MM-DD format
    pub async fn earnings_call_live(&self, from: &str, to: &str) -> Result<EarningsCallLive> {
        self.client
            .get(&format!("/stock/earnings-call-live?from={}&to={}", from, to))
            .await
    }

    /// Get investor presentations.
    ///
    /// List all investor presentations for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn presentations(&self, symbol: &str) -> Result<InvestorPresentations> {
        self.client
            .get(&format!("/stock/presentation?symbol={}", symbol))
            .await
    }

    /// Get price metrics.
    ///
    /// Get advanced price performance metrics for a stock.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn price_metrics(&self, symbol: &str) -> Result<PriceMetrics> {
        self.client
            .get(&format!("/stock/price-metric?symbol={}", symbol))
            .await
    }

    /// Get dividends v2.
    ///
    /// Get dividend data including dividend yield and ex-dividend dates.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    pub async fn dividends_v2(&self, symbol: &str) -> Result<DividendsV2> {
        self.client
            .get(&format!("/stock/dividend2?symbol={}", symbol))
            .await
    }
}
