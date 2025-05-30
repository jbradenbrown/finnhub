use finnhub::RateLimiter;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Verify that both rate limiting strategies maintain 30 req/s average
#[tokio::test]
async fn verify_rate_limit_averages() {
    println!("\n=== Verifying Rate Limit Averages ===\n");
    
    // Test 1: Verify per-second limiter over 10 seconds
    println!("Test 1: Per-second limiter over 10 seconds");
    let limiter = RateLimiter::finnhub_default();
    let start = Instant::now();
    let mut count = 0;
    
    // Try to make as many requests as possible in 10 seconds
    while start.elapsed() < Duration::from_secs(10) {
        if limiter.try_acquire().await.is_ok() {
            count += 1;
        } else {
            // Sleep a tiny bit to avoid busy waiting
            sleep(Duration::from_millis(10)).await;
        }
    }
    
    let elapsed = start.elapsed().as_secs_f64();
    let rate = count as f64 / elapsed;
    println!("  Made {} requests in {:.2}s", count, elapsed);
    println!("  Average rate: {:.2} req/s (should be ~30)", rate);
    assert!(rate >= 29.0 && rate <= 31.0, "Rate should be close to 30 req/s");
    
    // Test 2: Verify 15-second window limiter over 30 seconds
    println!("\nTest 2: 15-second window limiter over 30 seconds");
    let limiter = RateLimiter::finnhub_15s_window();
    let start = Instant::now();
    let mut count = 0;
    
    // Try to make as many requests as possible in 30 seconds
    while start.elapsed() < Duration::from_secs(30) {
        if limiter.try_acquire().await.is_ok() {
            count += 1;
        } else {
            // Sleep a tiny bit to avoid busy waiting
            sleep(Duration::from_millis(10)).await;
        }
    }
    
    let elapsed = start.elapsed().as_secs_f64();
    let rate = count as f64 / elapsed;
    println!("  Made {} requests in {:.2}s", count, elapsed);
    println!("  Average rate: {:.2} req/s (should be ~30)", rate);
    assert!(rate >= 29.0 && rate <= 31.0, "Rate should be close to 30 req/s");
    
    // Test 3: Verify burst behavior of 15-second window
    println!("\nTest 3: Burst behavior comparison");
    
    // Per-second limiter
    let limiter1 = RateLimiter::finnhub_default();
    let mut burst_count = 0;
    while limiter1.try_acquire().await.is_ok() && burst_count < 100 {
        burst_count += 1;
    }
    println!("  Per-second limiter: {} instant requests (should be 30)", burst_count);
    assert_eq!(burst_count, 30, "Should allow exactly 30 burst requests");
    
    // 15-second window limiter
    let limiter2 = RateLimiter::finnhub_15s_window();
    let mut burst_count = 0;
    while limiter2.try_acquire().await.is_ok() && burst_count < 500 {
        burst_count += 1;
    }
    println!("  15-second window: {} instant requests (should be 450)", burst_count);
    assert_eq!(burst_count, 450, "Should allow exactly 450 burst requests");
    
    println!("\n=== Summary ===");
    println!("✓ Both strategies maintain 30 req/s average over time");
    println!("✓ Per-second: 30 token burst, steady refill");
    println!("✓ 15-second: 450 token burst, same refill rate");
    println!("✓ The difference is in burst capacity, not average rate");
}

/// Test rate limit math
#[test]
fn test_rate_limit_math() {
    println!("\n=== Rate Limit Math ===\n");
    
    // Per-second math
    println!("Per-second rate limiting:");
    println!("  Capacity: 30 tokens");
    println!("  Refill: 30 tokens/second");
    println!("  Burst: 30 requests instantly");
    println!("  Then: 1 request every 33.33ms");
    
    // 15-second window math
    println!("\n15-second window rate limiting:");
    println!("  Capacity: 450 tokens (30 × 15)");
    println!("  Refill: 30 tokens/second (same!)");
    println!("  Burst: 450 requests instantly");
    println!("  Then: 1 request every 33.33ms");
    println!("  After 15 seconds of no requests: full 450 capacity again");
    
    // Examples
    println!("\nExample scenarios:");
    println!("1. Make 100 requests:");
    println!("   Per-second: 30 instant + 70 × 33.33ms = 2.33 seconds");
    println!("   15-second: 100 instant = 0 seconds");
    
    println!("\n2. Make 500 requests:");
    println!("   Per-second: 30 instant + 470 × 33.33ms = 15.67 seconds");
    println!("   15-second: 450 instant + 50 × 33.33ms = 1.67 seconds");
    
    println!("\n3. Continuous requests for 1 minute:");
    println!("   Both: ~1800 requests (30 req/s × 60s)");
}