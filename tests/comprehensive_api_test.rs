//! Comprehensive test of all Finnhub API endpoints.
//!
//! This test requires a valid API key and makes real API calls.
//! It will take several minutes to complete due to rate limiting.
//! Run with: FINNHUB_API_KEY=your_key cargo test test_all_endpoints -- --ignored --nocapture

use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy};
use std::time::Instant;

/// Test result tracking
struct TestResults {
    total: usize,
    passed: usize,
    failed: usize,
    errors: Vec<(String, String)>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    fn record_success(&mut self, endpoint: &str) {
        self.total += 1;
        self.passed += 1;
        println!("  ✓ {}", endpoint);
    }

    fn record_failure(&mut self, endpoint: &str, error: String) {
        self.total += 1;
        self.failed += 1;
        self.errors.push((endpoint.to_string(), error.clone()));
        println!("  ✗ {} - {}", endpoint, error);
    }

    fn print_summary(&self) {
        println!("\n=== Test Summary ===");
        println!("Total endpoints tested: {}", self.total);
        println!(
            "Passed: {} ({:.1}%)",
            self.passed,
            (self.passed as f64 / self.total as f64) * 100.0
        );
        println!(
            "Failed: {} ({:.1}%)",
            self.failed,
            (self.failed as f64 / self.total as f64) * 100.0
        );

        if !self.errors.is_empty() {
            println!("\nFailed endpoints:");
            for (endpoint, error) in &self.errors {
                println!("  - {}: {}", endpoint, error);
            }
        }
    }
}

#[tokio::test]
#[ignore = "Requires API key and makes many real API calls - takes several minutes"]
async fn test_all_endpoints() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");

    // Use 15-second window for better burst handling
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
    let client = FinnhubClient::with_config(api_key, config);

    let mut results = TestResults::new();
    let start_time = Instant::now();

    println!("\n=== Comprehensive Finnhub API Test ===");
    println!("Testing all endpoints with rate limiting...\n");

    // Test symbols to use
    let stock_symbol = "AAPL";
    let forex_pair = "OANDA:EUR_USD";
    let crypto_symbol = "BINANCE:BTCUSDT";
    let etf_symbol = "SPY";
    let _index_symbol = "^GSPC";

    // Stock endpoints
    println!("Testing Stock Endpoints:");

    // Quote and pricing
    test_endpoint(&client, &mut results, "stock.quote", || async {
        client.stock().quote(stock_symbol).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.candles", || async {
        use finnhub::models::stock::CandleResolution;
        let from = chrono::Utc::now().timestamp() - 86400 * 7; // 7 days ago
        let to = chrono::Utc::now().timestamp();
        client
            .stock()
            .candles(stock_symbol, CandleResolution::Daily, from, to)
            .await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.bid_ask", || async {
        client.stock().bid_ask(stock_symbol).await
    })
    .await;

    // Company information
    test_endpoint(&client, &mut results, "stock.company_profile", || async {
        client.stock().company_profile(stock_symbol).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.peers", || async {
        client.stock().peers(stock_symbol, None).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.executives", || async {
        client.stock().executives(stock_symbol).await
    })
    .await;

    // Financials
    test_endpoint(&client, &mut results, "stock.financials", || async {
        use finnhub::models::stock::{StatementFrequency, StatementType};
        client
            .stock()
            .financials(
                stock_symbol,
                StatementType::IncomeStatement,
                StatementFrequency::Annual,
            )
            .await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.metrics", || async {
        client.stock().metrics(stock_symbol).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.earnings", || async {
        client.stock().earnings(stock_symbol, None).await
    })
    .await;

    // Corporate actions
    test_endpoint(&client, &mut results, "stock.dividends", || async {
        let from = "2020-01-01";
        let to = "2024-12-31";
        client.stock().dividends(stock_symbol, from, to).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.splits", || async {
        let from = "2020-01-01";
        let to = "2024-12-31";
        client.stock().splits(stock_symbol, from, to).await
    })
    .await;

    // Analyst data
    test_endpoint(&client, &mut results, "stock.price_target", || async {
        client.stock().price_target(stock_symbol).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.recommendations", || async {
        client.stock().recommendations(stock_symbol).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.eps_estimates", || async {
        client
            .stock()
            .eps_estimates(stock_symbol, Some("quarterly"))
            .await
    })
    .await;

    // Insider trading
    test_endpoint(
        &client,
        &mut results,
        "stock.insider_transactions",
        || async { client.stock().insider_transactions(stock_symbol).await },
    )
    .await;

    test_endpoint(&client, &mut results, "stock.insider_sentiment", || async {
        let from = "2024-01-01";
        let to = "2024-12-31";
        client
            .stock()
            .insider_sentiment(stock_symbol, from, to)
            .await
    })
    .await;

    // Ownership
    test_endpoint(&client, &mut results, "stock.ownership", || async {
        client.stock().ownership(stock_symbol, None).await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.fund_ownership", || async {
        client.stock().fund_ownership(stock_symbol, None).await
    })
    .await;

    // Alternative data
    test_endpoint(&client, &mut results, "stock.social_sentiment", || async {
        let from = "2024-01-01";
        let to = "2024-01-31";
        client
            .stock()
            .social_sentiment(stock_symbol, from, to)
            .await
    })
    .await;

    test_endpoint(
        &client,
        &mut results,
        "stock.congressional_trading",
        || async {
            client
                .stock()
                .congressional_trading(stock_symbol, None, None)
                .await
        },
    )
    .await;

    test_endpoint(&client, &mut results, "stock.lobbying", || async {
        client.stock().lobbying(stock_symbol, None, None).await
    })
    .await;

    // ESG
    test_endpoint(&client, &mut results, "stock.esg", || async {
        client.stock().esg(stock_symbol).await
    })
    .await;

    // Market data
    test_endpoint(&client, &mut results, "stock.market_status", || async {
        client.stock().market_status("US").await
    })
    .await;

    test_endpoint(&client, &mut results, "stock.symbols", || async {
        client.stock().symbols("US").await
    })
    .await;

    // Forex endpoints
    println!("\nTesting Forex Endpoints:");

    test_endpoint(&client, &mut results, "forex.symbols", || async {
        client.forex().symbols("OANDA").await
    })
    .await;

    test_endpoint(&client, &mut results, "forex.rates", || async {
        client.forex().rates("USD").await
    })
    .await;

    test_endpoint(&client, &mut results, "forex.candles", || async {
        use finnhub::models::stock::CandleResolution;
        let from = chrono::Utc::now().timestamp() - 86400 * 7;
        let to = chrono::Utc::now().timestamp();
        client
            .forex()
            .candles(forex_pair, CandleResolution::Daily, from, to)
            .await
    })
    .await;

    // Crypto endpoints
    println!("\nTesting Crypto Endpoints:");

    test_endpoint(&client, &mut results, "crypto.exchanges", || async {
        client.crypto().exchanges().await
    })
    .await;

    test_endpoint(&client, &mut results, "crypto.symbols", || async {
        client.crypto().symbols("BINANCE").await
    })
    .await;

    test_endpoint(&client, &mut results, "crypto.candles", || async {
        use finnhub::models::stock::CandleResolution;
        let from = chrono::Utc::now().timestamp() - 86400;
        let to = chrono::Utc::now().timestamp();
        client
            .crypto()
            .candles(crypto_symbol, CandleResolution::SixtyMinutes, from, to)
            .await
    })
    .await;

    // ETF endpoints
    println!("\nTesting ETF Endpoints:");

    test_endpoint(&client, &mut results, "etf.profile", || async {
        client.etf().profile(Some(etf_symbol), None).await
    })
    .await;

    test_endpoint(&client, &mut results, "etf.holdings", || async {
        client
            .etf()
            .holdings(Some(etf_symbol), None, None, None)
            .await
    })
    .await;

    test_endpoint(&client, &mut results, "etf.country_exposure", || async {
        client.etf().country_exposure(Some(etf_symbol), None).await
    })
    .await;

    // News endpoints
    println!("\nTesting News Endpoints:");

    test_endpoint(&client, &mut results, "news.company_news", || async {
        let from = "2024-01-01";
        let to = "2024-01-31";
        client.news().company_news(stock_symbol, from, to).await
    })
    .await;

    test_endpoint(&client, &mut results, "news.market_news", || async {
        use finnhub::models::news::NewsCategory;
        client.news().market_news(NewsCategory::General, None).await
    })
    .await;

    // Calendar endpoints
    println!("\nTesting Calendar Endpoints:");

    test_endpoint(&client, &mut results, "calendar.earnings", || async {
        let from = "2024-01-01";
        let to = "2024-01-31";
        client.calendar().earnings(Some(from), Some(to), None).await
    })
    .await;

    test_endpoint(&client, &mut results, "calendar.ipo", || async {
        let from = "2024-01-01";
        let to = "2024-01-31";
        client.calendar().ipo(from, to).await
    })
    .await;

    // Economic data
    println!("\nTesting Economic Endpoints:");

    test_endpoint(&client, &mut results, "economic.codes", || async {
        client.economic().codes().await
    })
    .await;

    // Scanner endpoints
    println!("\nTesting Scanner Endpoints:");

    test_endpoint(
        &client,
        &mut results,
        "scanner.support_resistance",
        || async { client.scanner().support_resistance(stock_symbol, "D").await },
    )
    .await;

    test_endpoint(
        &client,
        &mut results,
        "scanner.aggregate_indicators",
        || async {
            client
                .scanner()
                .aggregate_indicators(stock_symbol, "D")
                .await
        },
    )
    .await;

    // Miscellaneous endpoints
    println!("\nTesting Misc Endpoints:");

    test_endpoint(&client, &mut results, "misc.country", || async {
        client.misc().country().await
    })
    .await;

    test_endpoint(&client, &mut results, "misc.symbol_search", || async {
        client.misc().symbol_search("apple", None).await
    })
    .await;

    test_endpoint(
        &client,
        &mut results,
        "misc.technical_indicator",
        || async {
            let from = chrono::Utc::now().timestamp() - 86400 * 30;
            let to = chrono::Utc::now().timestamp();
            let mut params = serde_json::Map::new();
            params.insert("timeperiod".to_string(), serde_json::json!(14));
            client
                .misc()
                .technical_indicator(
                    stock_symbol,
                    "D",
                    from,
                    to,
                    "sma",
                    Some(serde_json::Value::Object(params)),
                )
                .await
        },
    )
    .await;

    // Print final results
    let duration = start_time.elapsed();
    results.print_summary();

    println!(
        "\nTotal test duration: {:.2} seconds",
        duration.as_secs_f64()
    );
    println!(
        "Average time per endpoint: {:.2} seconds",
        duration.as_secs_f64() / results.total as f64
    );

    // Assert that most endpoints pass
    let pass_rate = results.passed as f64 / results.total as f64;
    assert!(pass_rate >= 0.8, "Less than 80% of endpoints passed");
}

/// Helper function to test an endpoint and record results
async fn test_endpoint<T, F, Fut>(
    _client: &FinnhubClient,
    results: &mut TestResults,
    name: &str,
    f: F,
) where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, finnhub::Error>>,
    T: std::fmt::Debug,
{
    match f().await {
        Ok(_) => results.record_success(name),
        Err(e) => {
            // Some errors are expected (e.g., no data for certain symbols/dates)
            let error_str = e.to_string();
            if error_str.contains("404") || error_str.contains("not found") {
                results.record_success(&format!("{} (no data)", name));
            } else {
                results.record_failure(name, error_str);
            }
        }
    }
}
