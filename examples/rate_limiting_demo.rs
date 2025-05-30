//! Demonstrates how the rate limiter works in the Finnhub client.
//!
//! This example shows:
//! - Token bucket behavior with 30 tokens capacity
//! - Continuous refill at 30 tokens per second
//! - Burst capability up to the capacity
//! - Blocking behavior when tokens are exhausted

use finnhub::rate_limiter::RateLimiter;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Finnhub Rate Limiter Demo ===\n");
    println!("The Finnhub API has a rate limit of 30 requests per second.");
    println!("This client uses a token bucket algorithm to enforce this limit.\n");
    
    let limiter = RateLimiter::finnhub_default();
    
    // Demo 1: Show initial capacity
    println!("1. Initial State");
    println!("   Available tokens: {}", limiter.available_tokens().await);
    println!("   This allows for burst requests up to the capacity.\n");
    
    // Demo 2: Burst capability
    println!("2. Burst Capability Demo");
    println!("   Making 30 requests as fast as possible...");
    let start = Instant::now();
    
    for i in 0..30 {
        limiter.acquire().await.unwrap();
        if i == 0 || i == 9 || i == 19 || i == 29 {
            println!("   Request {} completed at {:?}", i + 1, start.elapsed());
        }
    }
    
    println!("   All 30 requests completed in {:?}", start.elapsed());
    println!("   Available tokens: {}\n", limiter.available_tokens().await);
    
    // Demo 3: Refill demonstration
    println!("3. Token Refill Demo");
    println!("   Tokens refill at 30 per second...");
    
    for i in 0..3 {
        sleep(Duration::from_millis(333)).await;
        let tokens = limiter.available_tokens().await;
        println!("   After {}ms: {} tokens available", (i + 1) * 333, tokens);
    }
    
    println!();
    
    // Demo 4: Sustained rate
    println!("4. Sustained Rate Demo");
    println!("   Making requests at exactly 30/second for 2 seconds...");
    
    // First, let tokens refill
    println!("   Waiting for full refill...");
    sleep(Duration::from_secs(1)).await;
    
    let start = Instant::now();
    let mut request_count = 0;
    
    // Make requests at 30/second rate
    while start.elapsed() < Duration::from_secs(2) {
        limiter.acquire().await.unwrap();
        request_count += 1;
        
        // Sleep to maintain 30 req/s rate (33.33ms between requests)
        sleep(Duration::from_millis(33)).await;
    }
    
    println!("   Made {} requests in {:?}", request_count, start.elapsed());
    println!("   Available tokens: {}\n", limiter.available_tokens().await);
    
    // Demo 5: Rate limit enforcement
    println!("5. Rate Limit Enforcement Demo");
    println!("   Attempting to exceed the rate limit...");
    
    // Consume all tokens
    while limiter.try_acquire().await.is_ok() {
        // Consume silently
    }
    
    println!("   All tokens consumed. Available: {}", limiter.available_tokens().await);
    
    // Try to acquire when empty
    match limiter.try_acquire().await {
        Ok(_) => println!("   ERROR: Should have been rate limited!"),
        Err(e) => println!("   Rate limited as expected: {}", e),
    }
    
    // Show blocking behavior
    println!("   Waiting for next available token...");
    let start = Instant::now();
    limiter.acquire().await.unwrap();
    println!("   Token acquired after {:?}", start.elapsed());
    
    println!("\n=== Summary ===");
    println!("The rate limiter uses a token bucket algorithm:");
    println!("- Capacity: 30 tokens (allows burst up to 30 requests)");
    println!("- Refill rate: 30 tokens per second (continuous)");
    println!("- When tokens are exhausted, requests block until tokens are available");
    println!("- This ensures compliance with Finnhub's 30 requests/second limit");
    println!("\nIn practice, this means:");
    println!("- You can make up to 30 requests instantly if you haven't made any recently");
    println!("- After that, you're limited to 30 requests per second on average");
    println!("- The client handles rate limiting automatically - no manual throttling needed!");
}