use finnhub::{ClientConfig, FinnhubClient};
use std::time::{Duration, Instant};

/// Test to understand rate limiting behavior by timing actual requests
#[tokio::test]
#[ignore = "Requires API key"]
async fn test_rate_limiting_with_timing() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY must be set");
    
    println!("\n=== Rate Limiting Behavior Test ===\n");
    
    // Test 1: Default client (30 req/s internal limit)
    println!("Test 1: Default client with 30 req/s limit");
    let client = FinnhubClient::new(&api_key);
    
    println!("  Making 35 requests and timing them...");
    let start = Instant::now();
    let mut timings = Vec::new();
    
    for i in 0..35 {
        let req_start = start.elapsed();
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                timings.push((i + 1, req_start));
            }
            Err(e) => {
                println!("  Request {} failed: {}", i + 1, e);
                break;
            }
        }
    }
    
    // Print interesting timings
    println!("  Request 1 at {:?}", timings[0].1);
    println!("  Request 30 at {:?}", timings[29].1);
    println!("  Request 31 at {:?} (should be delayed)", timings[30].1);
    println!("  Request 35 at {:?}", timings[34].1);
    println!("  Total time: {:?}", start.elapsed());
    
    // Calculate if rate limiting kicked in
    let time_for_30 = timings[29].1;
    let time_for_31 = timings[30].1;
    let delay = time_for_31 - time_for_30;
    println!("  Delay between request 30 and 31: {:?}", delay);
    
    if delay > Duration::from_millis(10) {
        println!("  ✓ Internal rate limiter is working!");
    }
    
    println!("\nTest 2: High rate limit client (1000 req/s)");
    let mut config = ClientConfig::default();
    config.rate_limit = Some(1000);
    let client = FinnhubClient::with_config(&api_key, config);
    
    println!("  Making 100 requests with high limit...");
    let start = Instant::now();
    let mut success = 0;
    let mut api_rate_limited = false;
    
    for i in 0..100 {
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("429") || error_msg.contains("rate limit") {
                    println!("  API rate limit hit at request {}: {}", i + 1, e);
                    api_rate_limited = true;
                    break;
                } else {
                    println!("  Request {} failed: {}", i + 1, e);
                }
            }
        }
    }
    
    println!("  Completed {} requests in {:?}", success, start.elapsed());
    
    if api_rate_limited {
        println!("  ⚠️  Finnhub API enforces rate limiting server-side!");
        println!("  The internal rate limiter protects you from hitting this.");
    } else if success == 35 {
        println!("  ✓ All requests succeeded without hitting API limits");
        println!("  Note: Finnhub may still enforce limits over longer periods");
    }
    
    println!("\n=== Summary ===");
    println!("- Internal rate limiter can be configured via ClientConfig");
    println!("- Default: 30 requests/second (Finnhub's documented limit)");
    println!("- Can be increased for testing, but API may still enforce limits");
    println!("- Rate limiter uses token bucket: allows bursts, then throttles");
}