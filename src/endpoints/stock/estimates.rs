//! Earnings and revenue estimates endpoints.

use crate::{
    client::FinnhubClient,
    error::Result,
    models::stock::{
        DPSEstimates, EBITDAEstimates, EBITEstimates, EPSEstimates, EarningsQualityScore,
        GrossIncomeEstimates, NetIncomeEstimates, PretaxIncomeEstimates, RevenueEstimates,
    },
};

/// Earnings and revenue estimates endpoints.
pub struct EstimatesEndpoints<'a> {
    client: &'a FinnhubClient,
}

impl<'a> EstimatesEndpoints<'a> {
    /// Create a new estimates endpoints instance.
    pub fn new(client: &'a FinnhubClient) -> Self {
        Self { client }
    }

    /// Get EPS estimates.
    ///
    /// Returns analysts' EPS estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn eps(&self, symbol: &str, freq: Option<&str>) -> Result<EPSEstimates> {
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
    pub async fn revenue(&self, symbol: &str, freq: Option<&str>) -> Result<RevenueEstimates> {
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
    pub async fn ebitda(&self, symbol: &str, freq: Option<&str>) -> Result<EBITDAEstimates> {
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
    pub async fn ebit(&self, symbol: &str, freq: Option<&str>) -> Result<EBITEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = format!("/stock/ebit-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get net income estimates.
    ///
    /// Returns analysts' net income estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn net_income(&self, symbol: &str, freq: Option<&str>) -> Result<NetIncomeEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = format!("/stock/net-income-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get pretax income estimates.
    ///
    /// Returns analysts' pretax income estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn pretax_income(
        &self,
        symbol: &str,
        freq: Option<&str>,
    ) -> Result<PretaxIncomeEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = format!("/stock/pretax-income-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get gross income estimates.
    ///
    /// Returns analysts' gross income estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn gross_income(
        &self,
        symbol: &str,
        freq: Option<&str>,
    ) -> Result<GrossIncomeEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = format!("/stock/gross-income-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get dividend per share (DPS) estimates.
    ///
    /// Returns analysts' dividend per share estimates for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly (optional)
    pub async fn dps(&self, symbol: &str, freq: Option<&str>) -> Result<DPSEstimates> {
        let mut params = vec![format!("symbol={}", symbol)];

        if let Some(f) = freq {
            params.push(format!("freq={}", f));
        }

        let query = format!("/stock/dps-estimate?{}", params.join("&"));
        self.client.get(&query).await
    }

    /// Get earnings quality score.
    ///
    /// Returns earnings quality indicators for a company.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `freq` - Frequency: annual or quarterly
    pub async fn earnings_quality_score(
        &self,
        symbol: &str,
        freq: &str,
    ) -> Result<EarningsQualityScore> {
        self.client
            .get(&format!(
                "/stock/earnings-quality-score?symbol={}&freq={}",
                symbol, freq
            ))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientConfig, FinnhubClient, RateLimitStrategy};

    async fn test_client() -> FinnhubClient {
        dotenv::dotenv().ok();
        let api_key = std::env::var("FINNHUB_API_KEY").unwrap_or_else(|_| "test_key".to_string());

        let mut config = ClientConfig::default();
        config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
        FinnhubClient::with_config(api_key, config)
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_eps_estimates() {
        let client = test_client().await;
        let result = client.stock().eps_estimates("AAPL", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            assert!(!estimates.data.is_empty());

            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
                // Number of analysts should be positive if present
                if let Some(num) = estimate.number_analysts {
                    assert!(num > 0);
                }

                // Estimates should have reasonable values if present
                if let (Some(avg), Some(high), Some(low)) =
                    (estimate.eps_avg, estimate.eps_high, estimate.eps_low)
                {
                    assert!(avg != 0.0);
                    assert!(high >= avg);
                    assert!(low <= avg);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_eps_estimates_quarterly() {
        let client = test_client().await;
        let result = client
            .stock()
            .eps_estimates("MSFT", Some("quarterly"))
            .await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            assert_eq!(estimates.freq, Some("quarterly".to_string()));

            if !estimates.data.is_empty() {
                // Quarterly estimates should have Q1, Q2, Q3, or Q4 in period
                let first = &estimates.data[0];
                assert!(first.period.contains("Q") || first.period.len() == 10);
                // YYYY-MM-DD format
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_revenue_estimates() {
        let client = test_client().await;
        let result = client.stock().revenue_estimates("GOOGL", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            assert!(!estimates.data.is_empty());

            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
                // Number of analysts should be positive if present
                if let Some(num) = estimate.number_analysts {
                    assert!(num > 0);
                }

                // Revenue estimates should be positive and reasonable if present
                if let (Some(avg), Some(high), Some(low)) = (
                    estimate.revenue_avg,
                    estimate.revenue_high,
                    estimate.revenue_low,
                ) {
                    assert!(avg > 0.0);
                    assert!(high >= avg);
                    assert!(low <= avg);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_ebitda_estimates() {
        let client = test_client().await;
        let result = client
            .stock()
            .ebitda_estimates("AMZN", Some("annual"))
            .await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            assert_eq!(estimates.freq, Some("annual".to_string()));

            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
                // Number of analysts should be positive if present
                if let Some(num) = estimate.number_analysts {
                    assert!(num > 0);
                }

                // EBITDA should be reasonable if present
                if let (Some(avg), Some(high), Some(low)) = (
                    estimate.ebitda_avg,
                    estimate.ebitda_high,
                    estimate.ebitda_low,
                ) {
                    assert!(avg != 0.0);
                    assert!(high >= avg);
                    assert!(low <= avg);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_ebit_estimates() {
        let client = test_client().await;
        let result = client.stock().ebit_estimates("META", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());

            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
                // Number of analysts should be positive if present
                if let Some(num) = estimate.number_analysts {
                    assert!(num > 0);
                }

                // EBIT estimates validation if present
                if let (Some(avg), Some(high), Some(low)) =
                    (estimate.ebit_avg, estimate.ebit_high, estimate.ebit_low)
                {
                    assert!(avg != 0.0);
                    assert!(high >= avg);
                    assert!(low <= avg);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_earnings_quality_score() {
        let client = test_client().await;
        let result = client
            .stock()
            .earnings_quality_score("AAPL", "quarterly")
            .await;

        assert!(
            result.is_ok(),
            "Failed to get earnings quality score: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_net_income_estimates() {
        let client = test_client().await;
        let result = client.stock().net_income_estimates("AAPL", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
                if let (Some(avg), Some(high), Some(low)) = (
                    estimate.net_income_avg,
                    estimate.net_income_high,
                    estimate.net_income_low,
                ) {
                    assert!(high >= avg);
                    assert!(low <= avg);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_pretax_income_estimates() {
        let client = test_client().await;
        let result = client
            .stock()
            .pretax_income_estimates("AAPL", Some("annual"))
            .await;

        if let Ok(estimates) = result {
            assert_eq!(estimates.freq.as_deref(), Some("annual"));
            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_gross_income_estimates() {
        let client = test_client().await;
        let result = client.stock().gross_income_estimates("AAPL", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_dps_estimates() {
        let client = test_client().await;
        let result = client.stock().dps_estimates("AAPL", None).await;

        if let Ok(estimates) = result {
            assert!(!estimates.symbol.is_empty());
            for estimate in &estimates.data {
                assert!(!estimate.period.is_empty());
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_earnings_quality_score_annual() {
        let client = test_client().await;
        let result = client
            .stock()
            .earnings_quality_score("MSFT", "annual")
            .await;

        assert!(
            result.is_ok(),
            "Failed to get annual earnings quality score: {:?}",
            result.err()
        );
    }
}
