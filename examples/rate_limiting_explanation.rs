//! Explanation of how rate limiting works in the Finnhub client
//!
//! This example explains the rate limiting behavior without making API calls.

use finnhub::{ClientConfig, FinnhubClient};

fn main() {
    println!("=== Finnhub Rate Limiting Explanation ===\n");
    
    println!("How the rate limiter works:");
    println!("1. Token Bucket Algorithm:");
    println!("   - Starts with 30 tokens (burst capacity)");
    println!("   - Refills at 30 tokens per second continuously");
    println!("   - Each API request consumes 1 token");
    println!("   - If no tokens available, request waits\n");
    
    println!("2. Time-based behavior:");
    println!("   - NOT a sliding window (e.g., \"30 in last second\")");
    println!("   - NOT a fixed window (e.g., \"30 per calendar second\")");
    println!("   - It's a continuous refill system\n");
    
    println!("3. What this means in practice:");
    println!("   - You can burst 30 requests instantly if you haven't made any recently");
    println!("   - After that, you get 1 new request every 33.33ms (1/30 second)");
    println!("   - If you make 30 requests instantly, request #31 waits ~33ms");
    println!("   - If you wait 1 second, you can burst 30 more requests\n");
    
    println!("4. Configuration options:");
    println!("   a) Default client (30 req/s):");
    println!("      let client = FinnhubClient::new(api_key);");
    println!();
    println!("   b) Custom rate limit:");
    println!("      let mut config = ClientConfig::default();");
    println!("      config.rate_limit = Some(60);  // 60 req/s");
    println!("      let client = FinnhubClient::with_config(api_key, config);");
    println!();
    println!("   c) Effectively disable (set very high):");
    println!("      config.rate_limit = Some(10000);  // 10k req/s\n");
    
    println!("5. Why use the internal rate limiter?");
    println!("   - Prevents 429 errors from Finnhub's server");
    println!("   - More efficient than retrying failed requests");
    println!("   - Predictable behavior in your application");
    println!("   - Automatic handling - no manual sleep() needed\n");
    
    println!("6. Testing considerations:");
    println!("   - For unit tests: Use high rate limit to avoid delays");
    println!("   - For integration tests: Keep default to test real behavior");
    println!("   - Finnhub also enforces limits server-side");
    println!("   - Even with internal limiter disabled, API may still rate limit\n");
    
    // Show how to create clients with different configurations
    let api_key = "test-key";
    
    // Default client
    let _default_client = FinnhubClient::new(api_key);
    println!("Created default client with 30 req/s limit");
    
    // High rate client for testing
    let mut test_config = ClientConfig::default();
    test_config.rate_limit = Some(1000);
    let _test_client = FinnhubClient::with_config(api_key, test_config);
    println!("Created test client with 1000 req/s limit");
    
    // Custom rate client
    let mut custom_config = ClientConfig::default();
    custom_config.rate_limit = Some(10);
    let _custom_client = FinnhubClient::with_config(api_key, custom_config);
    println!("Created custom client with 10 req/s limit");
}