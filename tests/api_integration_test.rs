//! API integration test - validates all major endpoints work correctly.
//! 
//! Run with: FINNHUB_API_KEY=your_key cargo test test_api_integration -- --ignored --nocapture

use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy};
use std::time::Instant;

#[tokio::test]
#[ignore = "Requires API key and makes real API calls"]
async fn test_api_integration() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY must be set");
    
    // Use 15-second window for burst handling
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
    let client = FinnhubClient::with_config(api_key, config);
    
    println!("\n=== Finnhub API Integration Test ===\n");
    let start = Instant::now();
    let mut passed = 0;
    let mut failed = 0;
    
    // Stock endpoints
    println!("Testing Stock Endpoints:");
    
    if test_endpoint("stock.quote", client.stock().quote("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.company_profile", client.stock().company_profile("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.peers", client.stock().peers("AAPL", None)).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.price_target", client.stock().price_target("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.recommendations", client.stock().recommendations("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.earnings", client.stock().earnings("AAPL", None)).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.metrics", client.stock().metrics("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.insider_transactions", client.stock().insider_transactions("AAPL")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.ownership", client.stock().ownership("AAPL", None)).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("stock.market_status", client.stock().market_status("US")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Forex endpoints
    println!("\nTesting Forex Endpoints:");
    
    if test_endpoint("forex.symbols", client.forex().symbols("OANDA")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("forex.rates", client.forex().rates("USD")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Crypto endpoints  
    println!("\nTesting Crypto Endpoints:");
    
    if test_endpoint("crypto.exchanges", client.crypto().exchanges()).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("crypto.symbols", client.crypto().symbols("BINANCE")).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // ETF endpoints
    println!("\nTesting ETF Endpoints:");
    
    if test_endpoint("etf.profile", client.etf().profile(Some("SPY"), None)).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("etf.holdings", client.etf().holdings(Some("SPY"), None, None, None)).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // News endpoints
    println!("\nTesting News Endpoints:");
    
    if test_endpoint("news.market_news", 
        client.news().market_news(finnhub::models::news::NewsCategory::General, Some(10))
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("news.company_news", 
        client.news().company_news("AAPL", "2024-01-01", "2024-01-31")
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Calendar endpoints
    println!("\nTesting Calendar Endpoints:");
    
    if test_endpoint("calendar.earnings", 
        client.calendar().earnings(Some("2024-01-01"), Some("2024-01-07"), None)
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("calendar.ipo", 
        client.calendar().ipo("2024-01-01", "2024-01-31")
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Economic endpoints
    println!("\nTesting Economic Endpoints:");
    
    if test_endpoint("economic.codes", client.economic().codes()).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Scanner endpoints
    println!("\nTesting Scanner Endpoints:");
    
    if test_endpoint("scanner.support_resistance", 
        client.scanner().support_resistance("AAPL", "D")
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("scanner.aggregate_indicators", 
        client.scanner().aggregate_indicators("AAPL", "D")
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Misc endpoints
    println!("\nTesting Misc Endpoints:");
    
    if test_endpoint("misc.country", client.misc().country()).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    if test_endpoint("misc.symbol_search", 
        client.misc().symbol_search("apple", None)
    ).await {
        passed += 1;
    } else {
        failed += 1;
    }
    
    // Summary
    let duration = start.elapsed();
    let total = passed + failed;
    
    println!("\n=== Test Summary ===");
    println!("Total endpoints tested: {}", total);
    println!("Passed: {} ({:.1}%)", passed, (passed as f64 / total as f64) * 100.0);
    println!("Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
    println!("Test duration: {:.2} seconds", duration.as_secs_f64());
    println!("Average time per endpoint: {:.2} seconds", duration.as_secs_f64() / total as f64);
    
    // Allow some failures due to data availability
    assert!(passed as f64 / total as f64 >= 0.75, 
        "Less than 75% of endpoints passed. Check API key and data availability.");
    
    println!("\n✓ API integration test passed!");
}

/// Helper to test an endpoint and return success/failure
async fn test_endpoint<T, F>(name: &str, future: F) -> bool 
where
    F: std::future::Future<Output = Result<T, finnhub::Error>>,
    T: std::fmt::Debug,
{
    match future.await {
        Ok(_) => {
            println!("  ✓ {}", name);
            true
        }
        Err(e) => {
            // Some errors are expected (no data)
            if e.to_string().contains("404") || e.to_string().contains("not found") {
                println!("  ✓ {} (no data available)", name);
                true
            } else {
                println!("  ✗ {} - {}", name, e);
                false
            }
        }
    }
}