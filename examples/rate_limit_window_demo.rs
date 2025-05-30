//! Demonstrates using different rate limiting strategies with the Finnhub client.
//!
//! This example shows how to configure the client for different use cases:
//! - Standard per-second limiting for steady requests
//! - 15-second window for bursty workloads
//! - Custom rate limiting for specific needs

use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy};

#[tokio::main]
async fn main() {
    println!("=== Finnhub Rate Limiting Strategies Demo ===\n");

    // Example 1: Default per-second rate limiting
    println!("1. Default Rate Limiting (30 req/s)");
    println!("   Best for: Steady, continuous API usage");
    println!("   Behavior: Max 30 requests instantly, then 1 per 33.33ms");

    let _client = FinnhubClient::new("your-api-key");
    println!("   Created: FinnhubClient::new(api_key)\n");

    // Example 2: 15-second window rate limiting
    println!("2. 15-Second Window Rate Limiting (450 req/15s)");
    println!("   Best for: Bursty workloads, batch processing");
    println!("   Behavior: Max 450 requests instantly, still 30 req/s average");

    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
    let _client = FinnhubClient::with_config("your-api-key", config);
    println!("   Created with 15-second window\n");

    // Example 3: Custom rate limiting
    println!("3. Custom Rate Limiting");
    println!("   Best for: Specific requirements or testing");

    let mut config = ClientConfig::default();
    // Example: 100 burst capacity, 20 req/s refill
    config.rate_limit_strategy = RateLimitStrategy::Custom {
        capacity: 100,
        refill_rate: 20,
    };
    let _client = FinnhubClient::with_config("your-api-key", config);
    println!("   Created with 100 capacity, 20 req/s refill\n");

    // Example 4: Effectively disable rate limiting for testing
    println!("4. Disable Rate Limiting (for testing only!)");
    println!("   ⚠️  WARNING: May cause 429 errors from Finnhub API");

    let mut config = ClientConfig::default();
    config.rate_limit_strategy = RateLimitStrategy::Custom {
        capacity: 10000,
        refill_rate: 10000,
    };
    let _client = FinnhubClient::with_config("your-api-key", config);
    println!("   Created with very high limits\n");

    // Practical comparison
    println!("=== Practical Comparison ===\n");
    println!("Scenario: Process 100 stock quotes\n");

    println!("With standard rate limiting (30 req/s):");
    println!("  - First 30 requests: instant");
    println!("  - Remaining 70 requests: ~2.33 seconds");
    println!("  - Total time: ~2.33 seconds\n");

    println!("With 15-second window (450 req/15s):");
    println!("  - All 100 requests: instant");
    println!("  - Total time: ~0 seconds\n");

    println!("=== When to Use Each Strategy ===\n");

    println!("Use Per-Second (default) when:");
    println!("  - Making continuous requests throughout the day");
    println!("  - Want predictable, steady rate limiting");
    println!("  - Following Finnhub's documented limits strictly\n");

    println!("Use 15-Second Window when:");
    println!("  - Processing data in batches");
    println!("  - Need to fetch many symbols quickly");
    println!("  - Have periods of high activity followed by quiet periods");
    println!("  - Example: Market open data collection\n");

    println!("Use Custom when:");
    println!("  - Testing rate limit behavior");
    println!("  - Have specific requirements");
    println!("  - Using a different API tier with different limits\n");

    // Code example
    println!("=== Code Example ===");
    println!(
        r#"
// Standard usage
let client = FinnhubClient::new(api_key);

// 15-second window for batch processing
let mut config = ClientConfig::default();
config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
let client = FinnhubClient::with_config(api_key, config);

// Custom for testing (disable rate limiting)
let mut config = ClientConfig::default();
config.rate_limit_strategy = RateLimitStrategy::Custom {{
    capacity: 10000,
    refill_rate: 10000,
}};
let test_client = FinnhubClient::with_config(api_key, config);
"#
    );
}
