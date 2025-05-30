//! Error handling example demonstrating proper error management and recovery strategies.

use finnhub::{ClientConfig, Error, FinnhubClient, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚨 Error Handling Examples");
    println!("{}", "=".repeat(40));

    // Example 1: Invalid API key
    demonstrate_auth_errors().await;
    
    // Example 2: Rate limiting
    demonstrate_rate_limiting().await;
    
    // Example 3: Network errors
    demonstrate_network_errors().await;
    
    // Example 4: Data validation errors
    demonstrate_data_errors().await;
    
    // Example 5: Graceful degradation
    demonstrate_graceful_degradation().await?;
    
    // Example 6: Retry strategies
    demonstrate_retry_strategies().await;

    Ok(())
}

async fn demonstrate_auth_errors() {
    println!("\n🔐 Authentication Error Handling");
    println!("{}", "-".repeat(35));

    // Create client with invalid API key
    let invalid_client = FinnhubClient::new("invalid_api_key");

    match invalid_client.stock().quote("AAPL").await {
        Ok(_) => println!("Unexpected success with invalid key"),
        Err(Error::Unauthorized) => {
            println!("✅ Caught unauthorized error - invalid API key");
            println!("   Action: Prompt user to check API key configuration");
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    // Demonstrate proper error context
    println!("\n💡 Best Practice: Always handle authentication errors gracefully");
    println!("   - Check API key validity before making requests");
    println!("   - Provide clear error messages to users");
    println!("   - Consider API key rotation strategies");
}

async fn demonstrate_rate_limiting() {
    println!("\n⏱️ Rate Limiting Error Handling");
    println!("{}", "-".repeat(35));

    // Create client with very low rate limit for demonstration
    let config = ClientConfig {
        rate_limit: Some(1), // Very restrictive: 1 request per second
        ..ClientConfig::default()
    };

    // Use a valid API key if available, otherwise skip this demo
    if let Ok(api_key) = std::env::var("FINNHUB_API_KEY") {
        let rate_limited_client = FinnhubClient::with_config(api_key, config);

        println!("Making rapid requests to trigger rate limiting...");

        // Make multiple rapid requests
        for i in 1..=3 {
            match rate_limited_client.stock().quote("AAPL").await {
                Ok(quote) => {
                    println!("✅ Request {}: Success - Price: ${:.2}", i, quote.current_price);
                }
                Err(Error::RateLimitExceeded { retry_after }) => {
                    println!("⏸️ Request {}: Rate limited - Retry after {} seconds", i, retry_after);
                    
                    // Demonstrate proper retry handling
                    println!("   Waiting {} seconds before retry...", retry_after);
                    tokio::time::sleep(Duration::from_secs(retry_after)).await;
                    
                    // Retry the request
                    match rate_limited_client.stock().quote("AAPL").await {
                        Ok(quote) => println!("✅ Retry successful - Price: ${:.2}", quote.current_price),
                        Err(e) => println!("❌ Retry failed: {}", e),
                    }
                    break;
                }
                Err(e) => {
                    println!("❌ Request {}: Unexpected error: {}", i, e);
                }
            }
        }
    } else {
        println!("⚠️ FINNHUB_API_KEY not set - skipping rate limit demo");
    }

    println!("\n💡 Best Practice: Implement exponential backoff for rate limits");
    println!("   - Respect the retry-after header");
    println!("   - Use jitter to avoid thundering herd");
    println!("   - Consider request queuing for high-volume applications");
}

async fn demonstrate_network_errors() {
    println!("\n🌐 Network Error Handling");
    println!("{}", "-".repeat(35));

    // Create client with invalid base URL to simulate network issues
    let config = ClientConfig {
        base_url: "https://invalid-domain-that-does-not-exist.com".to_string(),
        ..ClientConfig::default()
    };

    let network_client = FinnhubClient::with_config("test_key", config);

    match network_client.stock().quote("AAPL").await {
        Ok(_) => println!("Unexpected success with invalid URL"),
        Err(Error::Http(http_err)) => {
            println!("✅ Caught network error: {}", http_err);
            
            // Categorize the error
            if http_err.is_timeout() {
                println!("   Type: Timeout - Network request took too long");
                println!("   Action: Retry with exponential backoff");
            } else if http_err.is_connect() {
                println!("   Type: Connection - Unable to reach server");
                println!("   Action: Check network connectivity");
            } else {
                println!("   Type: Other HTTP error");
                println!("   Action: Log error and retry later");
            }
        }
        Err(e) => println!("❌ Unexpected error type: {}", e),
    }

    println!("\n💡 Best Practice: Implement robust network error handling");
    println!("   - Distinguish between retryable and non-retryable errors");
    println!("   - Implement circuit breaker pattern for failing services");
    println!("   - Provide offline mode or cached data when possible");
}

async fn demonstrate_data_errors() {
    println!("\n📊 Data Validation Error Handling");
    println!("{}", "-".repeat(35));

    // This example shows how to handle cases where API returns unexpected data
    if let Ok(api_key) = std::env::var("FINNHUB_API_KEY") {
        let client = FinnhubClient::new(api_key);

        // Try to get data for an invalid symbol
        match client.stock().quote("INVALID_SYMBOL_XYZ").await {
            Ok(quote) => {
                // Even if the request succeeds, validate the data
                if quote.current_price <= 0.0 {
                    println!("⚠️ Received invalid price data: {}", quote.current_price);
                    println!("   Action: Use fallback data source or cached values");
                } else {
                    println!("✅ Valid data received for invalid symbol (unexpected)");
                }
            }
            Err(Error::ApiError { status, message }) => {
                println!("✅ API returned error for invalid symbol:");
                println!("   Status: {}", status);
                println!("   Message: {}", message);
                println!("   Action: Validate symbols before making requests");
            }
            Err(e) => println!("❌ Unexpected error: {}", e),
        }
    } else {
        println!("⚠️ FINNHUB_API_KEY not set - simulating data validation");
        
        // Simulate data validation without API call
        let simulated_price = -10.0; // Invalid negative price
        if simulated_price <= 0.0 {
            println!("⚠️ Detected invalid price data: {}", simulated_price);
            println!("   Action: Use default value or request fresh data");
        }
    }

    println!("\n💡 Best Practice: Always validate received data");
    println!("   - Check for reasonable value ranges");
    println!("   - Validate required fields are present");
    println!("   - Implement data sanitization");
}

async fn demonstrate_graceful_degradation() -> Result<()> {
    println!("\n🛡️ Graceful Degradation Strategies");
    println!("{}", "-".repeat(35));

    if let Ok(api_key) = std::env::var("FINNHUB_API_KEY") {
        let client = FinnhubClient::new(api_key);

        // Primary data source
        let quote_result = client.stock().quote("AAPL").await;
        
        // Fallback chain
        let final_data = match quote_result {
            Ok(quote) => {
                println!("✅ Primary data source successful");
                format!("Live price: ${:.2}", quote.current_price)
            }
            Err(e) => {
                println!("⚠️ Primary data source failed: {}", e);
                
                // Try alternative endpoint
                match client.stock().company_profile("AAPL").await {
                    Ok(profile) => {
                        println!("✅ Fallback to company profile data");
                        format!("Company: {}", profile.name.unwrap_or_else(|| "Apple Inc".to_string()))
                    }
                    Err(e) => {
                        println!("⚠️ Secondary data source failed: {}", e);
                        
                        // Final fallback to cached/static data
                        println!("✅ Using cached/static data");
                        "AAPL - Apple Inc (cached data)".to_string()
                    }
                }
            }
        };

        println!("📊 Final result: {}", final_data);
    } else {
        println!("⚠️ FINNHUB_API_KEY not set - using cached data");
        println!("📊 Final result: AAPL - Apple Inc (cached data)");
    }

    println!("\n💡 Best Practice: Design fallback chains");
    println!("   - Primary -> Secondary -> Cached -> Static");
    println!("   - Degrade functionality gracefully");
    println!("   - Inform users about data freshness");

    Ok(())
}

async fn demonstrate_retry_strategies() {
    println!("\n🔄 Retry Strategy Implementation");
    println!("{}", "-".repeat(35));

    if let Ok(api_key) = std::env::var("FINNHUB_API_KEY") {
        let client = FinnhubClient::new(api_key);

        // Implement exponential backoff retry
        let max_retries = 3;
        let mut retry_count = 0;
        let base_delay = Duration::from_millis(100);

        loop {
            match client.stock().quote("AAPL").await {
                Ok(quote) => {
                    println!("✅ Request successful after {} retries", retry_count);
                    println!("   Price: ${:.2}", quote.current_price);
                    break;
                }
                Err(Error::RateLimitExceeded { retry_after }) => {
                    println!("⏸️ Rate limited - waiting {} seconds", retry_after);
                    tokio::time::sleep(Duration::from_secs(retry_after)).await;
                    // Don't count rate limit as a retry
                    continue;
                }
                Err(Error::Http(_)) if retry_count < max_retries => {
                    retry_count += 1;
                    let delay = base_delay * 2_u32.pow(retry_count - 1); // Exponential backoff
                    println!("⚠️ Request failed - retry {}/{} after {:?}", 
                        retry_count, max_retries, delay);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    println!("❌ Request failed permanently: {}", e);
                    break;
                }
            }
        }
    } else {
        println!("⚠️ FINNHUB_API_KEY not set - simulating retry logic");
        
        for attempt in 1..=3 {
            let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
            println!("🔄 Attempt {} - would wait {:?} on failure", attempt, delay);
        }
    }

    println!("\n💡 Best Practice: Smart retry strategies");
    println!("   - Use exponential backoff with jitter");
    println!("   - Set maximum retry limits");
    println!("   - Don't retry on authentication errors");
    println!("   - Log retry attempts for monitoring");
}

// Helper function to demonstrate error pattern matching
#[allow(dead_code)]
fn handle_finnhub_error(error: Error) -> String {
    match error {
        Error::Unauthorized => {
            "Invalid API key - please check your credentials".to_string()
        }
        Error::RateLimitExceeded { retry_after } => {
            format!("Rate limit exceeded - retry after {} seconds", retry_after)
        }
        Error::ApiError { status, message } => {
            format!("API error {}: {}", status, message)
        }
        Error::Http(http_err) => {
            if http_err.is_timeout() {
                "Request timed out - please try again".to_string()
            } else if http_err.is_connect() {
                "Connection failed - check your internet connection".to_string()
            } else {
                format!("Network error: {}", http_err)
            }
        }
        Error::Deserialization(serde_err) => {
            format!("Data parsing error: {}", serde_err)
        }
        Error::InvalidParameter(msg) => {
            format!("Invalid parameter: {}", msg)
        }
        Error::InvalidRequest(msg) => {
            format!("Invalid request: {}", msg)
        }
        Error::UrlParse(url_err) => {
            format!("URL parsing error: {}", url_err)
        }
        Error::Internal(msg) => {
            format!("Internal error: {}", msg)
        }
        Error::Timeout => {
            "Request timed out".to_string()
        }
    }
}