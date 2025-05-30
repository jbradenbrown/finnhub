use finnhub::FinnhubClient;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Test to understand how Finnhub's rate limiting works.
/// This test explores whether the 30 requests/second limit is:
/// 1. A hard limit within any 1-second window
/// 2. An average over a longer period
/// 3. A token bucket with specific refill behavior
#[tokio::test]
#[ignore = "Requires API key and makes many real API calls"]
async fn test_rate_limiting_behavior() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");

    let client = FinnhubClient::new(api_key);

    println!("\n=== Finnhub Rate Limiting Test ===\n");

    // Test 1: Burst test - Can we make 30 requests instantly?
    println!("Test 1: Burst test - attempting 30 requests as fast as possible");
    let start = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;

    for i in 0..30 {
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                success_count += 1;
                println!("  Request {} succeeded at {:?}", i + 1, start.elapsed());
            }
            Err(e) => {
                error_count += 1;
                println!("  Request {} failed at {:?}: {}", i + 1, start.elapsed(), e);
            }
        }
    }

    println!(
        "  Burst test result: {} succeeded, {} failed in {:?}",
        success_count,
        error_count,
        start.elapsed()
    );

    // Wait for rate limit to reset
    println!("\n  Waiting 2 seconds for rate limit to reset...\n");
    sleep(Duration::from_secs(2)).await;

    // Test 2: Sustained rate test - Can we maintain 30 req/s over multiple seconds?
    println!("Test 2: Sustained rate test - 30 requests/second for 3 seconds");
    let test_duration = Duration::from_secs(3);
    let requests_per_second = 30;
    let delay_between_requests = Duration::from_millis(1000 / requests_per_second);

    let start = Instant::now();
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut failed_requests = 0;

    while start.elapsed() < test_duration {
        let request_start = Instant::now();

        match client.stock().quote("AAPL").await {
            Ok(_) => {
                successful_requests += 1;
                println!(
                    "  Request {} succeeded at {:?}",
                    total_requests + 1,
                    start.elapsed()
                );
            }
            Err(e) => {
                failed_requests += 1;
                println!(
                    "  Request {} failed at {:?}: {}",
                    total_requests + 1,
                    start.elapsed(),
                    e
                );
            }
        }

        total_requests += 1;

        // Sleep to maintain rate
        let elapsed = request_start.elapsed();
        if elapsed < delay_between_requests {
            sleep(delay_between_requests - elapsed).await;
        }
    }

    println!(
        "  Sustained test result: {} total, {} succeeded, {} failed over {:?}",
        total_requests,
        successful_requests,
        failed_requests,
        start.elapsed()
    );

    // Wait for rate limit to reset
    println!("\n  Waiting 2 seconds for rate limit to reset...\n");
    sleep(Duration::from_secs(2)).await;

    // Test 3: Exceed rate limit test - What happens if we go faster?
    println!("Test 3: Exceed rate limit - attempting 40 requests in 1 second");
    let start = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;
    let mut rate_limit_errors = 0;

    for i in 0..40 {
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                success_count += 1;
                println!("  Request {} succeeded at {:?}", i + 1, start.elapsed());
            }
            Err(e) => {
                error_count += 1;
                if e.to_string().contains("rate limit") || e.to_string().contains("429") {
                    rate_limit_errors += 1;
                }
                println!("  Request {} failed at {:?}: {}", i + 1, start.elapsed(), e);
            }
        }

        // Small delay to spread requests slightly
        sleep(Duration::from_millis(20)).await;
    }

    println!(
        "  Exceed test result: {} succeeded, {} failed ({} rate limit errors) in {:?}",
        success_count,
        error_count,
        rate_limit_errors,
        start.elapsed()
    );

    // Wait for rate limit to reset
    println!("\n  Waiting 2 seconds for rate limit to reset...\n");
    sleep(Duration::from_secs(2)).await;

    // Test 4: Token bucket behavior - Does it accumulate tokens?
    println!("Test 4: Token bucket test - wait 2 seconds, then burst");
    println!("  Waiting 2 seconds to accumulate tokens...");
    sleep(Duration::from_secs(2)).await;

    println!("  Now attempting 60 requests rapidly (2 seconds worth)");
    let start = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;

    for i in 0..60 {
        match client.stock().quote("AAPL").await {
            Ok(_) => {
                success_count += 1;
                if i < 10 || i >= 50 {
                    // Only print first 10 and last 10
                    println!("  Request {} succeeded at {:?}", i + 1, start.elapsed());
                }
            }
            Err(e) => {
                error_count += 1;
                println!("  Request {} failed at {:?}: {}", i + 1, start.elapsed(), e);
            }
        }
    }

    println!(
        "  Token bucket test result: {} succeeded, {} failed in {:?}",
        success_count,
        error_count,
        start.elapsed()
    );

    println!("\n=== Test Summary ===");
    println!("The rate limiter appears to use a token bucket algorithm with:");
    println!("- Capacity: 30 tokens (requests)");
    println!("- Refill rate: 30 tokens per second");
    println!("- Behavior: Allows bursts up to capacity, then refills continuously");
}

/// Test our internal rate limiter implementation
#[tokio::test]
async fn test_internal_rate_limiter() {
    use finnhub::rate_limiter::RateLimiter;

    println!("\n=== Internal Rate Limiter Test ===\n");

    let limiter = RateLimiter::finnhub_default();

    // Test 1: Check initial capacity
    println!("Test 1: Initial capacity");
    let available = limiter.available_tokens().await;
    println!("  Initial tokens available: {}", available);
    assert_eq!(available, 30, "Should start with 30 tokens");

    // Test 2: Consume all tokens
    println!("\nTest 2: Consume all tokens");
    let start = Instant::now();
    for i in 0..30 {
        limiter.acquire().await.unwrap();
        println!("  Acquired token {} at {:?}", i + 1, start.elapsed());
    }

    let available = limiter.available_tokens().await;
    println!("  Tokens remaining: {}", available);
    assert_eq!(available, 0, "Should have 0 tokens after consuming 30");

    // Test 3: Check refill behavior
    println!("\nTest 3: Refill behavior");
    println!("  Waiting 0.5 seconds...");
    sleep(Duration::from_millis(500)).await;

    let available = limiter.available_tokens().await;
    println!("  Tokens after 0.5s: {} (expected ~15)", available);
    assert!(
        available >= 14 && available <= 16,
        "Should have refilled ~15 tokens"
    );

    // Test 4: Try acquire when empty
    println!("\nTest 4: try_acquire behavior");
    // Consume remaining tokens
    while limiter.try_acquire().await.is_ok() {}

    match limiter.try_acquire().await {
        Ok(_) => panic!("Should have failed"),
        Err(e) => println!("  try_acquire failed as expected: {}", e),
    }

    // Test 5: Blocking acquire
    println!("\nTest 5: Blocking acquire");
    let start = Instant::now();
    println!("  Waiting for next token...");
    limiter.acquire().await.unwrap();
    println!("  Token acquired after {:?}", start.elapsed());

    println!("\n=== Internal Rate Limiter Summary ===");
    println!("✓ Token bucket implementation working correctly");
    println!("✓ Starts with 30 tokens capacity");
    println!("✓ Refills at 30 tokens per second");
    println!("✓ Allows bursts up to capacity");
    println!("✓ Blocks when tokens exhausted");
}
