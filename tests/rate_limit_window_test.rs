use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy, RateLimiter};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Test demonstrating the difference between per-second and 15-second window rate limiting
#[tokio::test]
async fn test_rate_limit_strategies() {
    println!("\n=== Rate Limiting Strategy Comparison ===\n");
    
    // Test 1: Standard per-second rate limiting
    println!("Test 1: Standard per-second rate limiting (30 req/s)");
    let limiter = RateLimiter::finnhub_default();
    
    println!("  Initial tokens: {}", limiter.available_tokens().await);
    
    // Consume all 30 tokens
    for _ in 0..30 {
        limiter.acquire().await.unwrap();
    }
    
    println!("  After 30 requests: {} tokens", limiter.available_tokens().await);
    
    // Wait 0.5 seconds
    sleep(Duration::from_millis(500)).await;
    println!("  After 0.5s: {} tokens (should be ~15)", limiter.available_tokens().await);
    
    // Test 2: 15-second window rate limiting
    println!("\nTest 2: 15-second window rate limiting (450 req/15s)");
    let limiter = RateLimiter::finnhub_15s_window();
    
    println!("  Initial tokens: {}", limiter.available_tokens().await);
    
    // Can make 450 requests immediately
    let start = Instant::now();
    for i in 0..100 {
        limiter.acquire().await.unwrap();
        if i == 49 {
            println!("  50 requests completed at {:?}", start.elapsed());
        }
    }
    println!("  100 requests completed at {:?}", start.elapsed());
    println!("  Remaining tokens: {}", limiter.available_tokens().await);
    
    // Wait 1 second and check refill
    sleep(Duration::from_secs(1)).await;
    let tokens = limiter.available_tokens().await;
    println!("  After 1s: {} tokens (should have added ~30)", tokens);
    
    println!("\n=== Summary ===");
    println!("Per-second (default):");
    println!("  - 30 token capacity");
    println!("  - Good for steady rate limiting");
    println!("  - Prevents bursts larger than 30");
    println!("\n15-second window:");
    println!("  - 450 token capacity (30 * 15)");
    println!("  - Allows larger bursts");
    println!("  - Still averages to 30 req/s over time");
    println!("  - Better for bursty workloads");
}

/// Test using rate limit strategies with the client
#[tokio::test]
async fn test_client_with_strategies() {
    println!("\n=== Client Rate Limiting Strategies ===\n");
    
    // Example 1: Default per-second client
    println!("Example 1: Default client (per-second)");
    let _client1 = FinnhubClient::new("test-key");
    println!("  Created client with 30 req/s limit");
    
    // Example 2: 15-second window client
    println!("\nExample 2: 15-second window client");
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
    let _client2 = FinnhubClient::with_config("test-key", config);
    println!("  Created client with 450 req/15s limit");
    
    // Example 3: Custom rate limiting
    println!("\nExample 3: Custom rate limiting");
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::Custom {
        capacity: 100,
        refill_rate: 20,
    };
    let _client3 = FinnhubClient::with_config("test-key", config);
    println!("  Created client with 100 capacity, 20 req/s refill");
    
    // Example 4: Legacy rate_limit field (for backwards compatibility)
    println!("\nExample 4: Legacy rate_limit field");
    let mut config = ClientConfig::default();
    config.rate_limit = Some(60); // 60 req/s
    let _client4 = FinnhubClient::with_config("test-key", config);
    println!("  Created client with 60 req/s limit (legacy)");
}

/// Practical example: Burst processing with 15-second window
#[tokio::test]
#[ignore = "Requires API key"]
async fn test_burst_processing() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY must be set");
    
    println!("\n=== Burst Processing Example ===\n");
    
    // Create client with 15-second window
    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
    let client = FinnhubClient::with_config(api_key, config);
    
    println!("Scenario: Process 100 stocks quickly, then pause");
    
    let stocks = vec!["AAPL", "GOOGL", "MSFT", "AMZN", "FB", "TSLA", "NVDA", "JPM", "JNJ", "V"];
    let start = Instant::now();
    let mut count = 0;
    
    // Process 100 requests (10 stocks * 10 times)
    for round in 0..10 {
        for stock in &stocks {
            match client.stock().quote(stock).await {
                Ok(_) => count += 1,
                Err(e) => println!("Error: {}", e),
            }
        }
        if round == 0 {
            println!("  First 10 requests at {:?}", start.elapsed());
        }
    }
    
    println!("  Processed {} requests in {:?}", count, start.elapsed());
    println!("  With per-second limiting, this would take ~3.3 seconds");
    println!("  With 15-second window, it's nearly instant!");
}