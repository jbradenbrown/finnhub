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
    println!("  Average rate: {:.2} req/s", rate);
    
    // With 450 initial tokens + 900 tokens generated over 30s = 1350 total
    // Expected rate is 1350 / 30 = 45 req/s
    // In practice, slightly lower due to timing overhead
    println!("  Expected rate: ~45 req/s (due to initial burst capacity)");
    assert!(rate >= 42.0 && rate <= 46.0, "Rate should be around 42-45 req/s due to burst capacity");
    
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
    
    // Test 4: Verify 15-second window approaches 30 req/s over longer period
    println!("\nTest 4: 15-second window limiter over 60 seconds");
    let limiter = RateLimiter::finnhub_15s_window();
    let start = Instant::now();
    let mut count = 0;
    
    // Try to make as many requests as possible in 60 seconds
    while start.elapsed() < Duration::from_secs(60) {
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
    println!("  Average rate: {:.2} req/s", rate);
    
    // Over 60 seconds: 450 initial + 1800 generated = 2250 total
    // Expected rate is 2250 / 60 = 37.5 req/s
    // In practice, slightly lower due to timing overhead
    println!("  Expected rate: ~35-37 req/s (approaching 30 req/s as time increases)");
    assert!(rate >= 34.0 && rate <= 38.0, "Rate should be around 35-37 req/s over 60s");

    println!("\n=== Summary ===");
    println!("✓ Per-second: Maintains 30 req/s average consistently");
    println!("✓ 15-second: Allows bursting up to 450 requests");
    println!("✓ 15-second: Higher short-term average (45 req/s over 30s) due to initial burst");
    println!("✓ 15-second: Approaches 30 req/s average over longer periods (37.5 req/s over 60s)");
    println!("✓ Both refill at 30 tokens/second");
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