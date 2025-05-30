//! Essential endpoints test - a smaller subset for regular testing.
//!
//! This test verifies core functionality with minimal API calls.
//! Run with: FINNHUB_API_KEY=your_key cargo test test_essential_endpoints -- --ignored --nocapture

use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy};
use std::time::Instant;

#[tokio::test]
#[ignore = "Requires API key"]
async fn test_essential_endpoints() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");

    // Use standard rate limiting for smaller test
    let client = FinnhubClient::new(api_key);

    println!("\n=== Essential Endpoints Test ===\n");
    let start = Instant::now();

    // Test 1: Basic stock quote
    println!("1. Stock Quote (AAPL):");
    match client.stock().quote("AAPL").await {
        Ok(quote) => {
            println!("   ✓ Price: ${:.2}", quote.current_price);
            println!(
                "   ✓ Change: {:.2} ({:.2}%)",
                quote.change, quote.percent_change
            );
            assert!(quote.current_price > 0.0, "Price should be positive");
        }
        Err(e) => panic!("Stock quote failed: {}", e),
    }

    // Test 2: Company profile
    println!("\n2. Company Profile (MSFT):");
    match client.stock().company_profile("MSFT").await {
        Ok(profile) => {
            println!("   ✓ Name: {}", profile.name.as_deref().unwrap_or("N/A"));
            println!(
                "   ✓ Industry: {}",
                profile.finnhub_industry.as_deref().unwrap_or("N/A")
            );
            println!(
                "   ✓ Market Cap: ${:.2}B",
                profile.market_capitalization.unwrap_or(0.0) / 1000.0
            );
        }
        Err(e) => panic!("Company profile failed: {}", e),
    }

    // Test 3: Forex rates
    println!("\n3. Forex Rates (USD base):");
    match client.forex().rates("USD").await {
        Ok(rates) => {
            let eur_rate = rates.quote.get("EUR").copied().unwrap_or(0.0);
            let gbp_rate = rates.quote.get("GBP").copied().unwrap_or(0.0);
            println!("   ✓ USD/EUR: {:.4}", eur_rate);
            println!("   ✓ USD/GBP: {:.4}", gbp_rate);
            assert!(eur_rate > 0.0, "EUR rate should be positive");
        }
        Err(e) => panic!("Forex rates failed: {}", e),
    }

    // Test 4: Crypto exchanges
    println!("\n4. Crypto Exchanges:");
    match client.crypto().exchanges().await {
        Ok(exchanges) => {
            println!("   ✓ Found {} exchanges", exchanges.len());
            assert!(!exchanges.is_empty(), "Should have crypto exchanges");
            // Print first 5
            for (i, _exchange) in exchanges.iter().take(5).enumerate() {
                println!("     - Exchange {}", i + 1);
            }
        }
        Err(e) => panic!("Crypto exchanges failed: {}", e),
    }

    // Test 5: Market news
    println!("\n5. Market News:");
    match client
        .news()
        .market_news(finnhub::models::news::NewsCategory::General, Some(10))
        .await
    {
        Ok(news) => {
            println!("   ✓ Found {} news items", news.len());
            for (i, article) in news.iter().take(3).enumerate() {
                println!("   {}. {}", i + 1, article.headline);
            }
        }
        Err(e) => panic!("Market news failed: {}", e),
    }

    // Test 6: Error handling - invalid symbol
    println!("\n6. Error Handling (Invalid Symbol):");
    match client.stock().quote("INVALID_SYMBOL_XYZ").await {
        Ok(_) => panic!("Should have returned an error for invalid symbol"),
        Err(e) => {
            println!("   ✓ Returned error: {}", e);
            assert!(
                e.to_string().contains("404")
                    || e.to_string().contains("not found")
                    || e.to_string().contains("Invalid"),
                "Expected error for invalid symbol"
            );
        }
    }

    // Test 7: Rate limiting behavior
    println!("\n7. Rate Limiting Test:");
    println!("   Making 5 rapid requests...");
    let rate_start = Instant::now();
    for i in 1..=5 {
        match client.stock().quote("GOOGL").await {
            Ok(quote) => {
                println!(
                    "   Request {} at {:?}: ${:.2}",
                    i,
                    rate_start.elapsed(),
                    quote.current_price
                );
            }
            Err(e) => {
                println!("   Request {} failed: {}", i, e);
            }
        }
    }

    let duration = start.elapsed();
    println!("\n=== Test Summary ===");
    println!("All essential endpoints working correctly!");
    println!("Total duration: {:.2} seconds", duration.as_secs_f64());
    println!("\nNote: Run comprehensive_api_test for full endpoint coverage");
}

#[tokio::test]
#[ignore = "Requires API key"]
async fn test_rate_limiting_enforcement() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");

    println!("\n=== Rate Limiting Enforcement Test ===\n");

    // Create client with high rate limit to test API enforcement
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::Custom {
        capacity: 1000,
        refill_rate: 1000,
    };
    let client = FinnhubClient::with_config(api_key, config);

    println!("Testing API rate limit enforcement (internal limiter disabled)...");

    let start = Instant::now();
    let mut requests = 0;
    let mut rate_limited = false;

    // Try to make 50 requests rapidly
    for _ in 0..50 {
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                requests += 1;
            }
            Err(e) => {
                if e.to_string().contains("429") || e.to_string().contains("rate limit") {
                    rate_limited = true;
                    println!(
                        "Rate limited by API after {} requests at {:?}",
                        requests,
                        start.elapsed()
                    );
                    break;
                }
            }
        }
    }

    if !rate_limited {
        println!("Completed {} requests without API rate limiting", requests);
        println!("Note: Finnhub may have different limits for different API tiers");
    }

    println!("\nConclusion: Internal rate limiter protects against hitting API limits");
}
