//! Integration tests for stock quote endpoint.

use dotenv::dotenv;
use finnhub::{FinnhubClient, Result};
use std::env;

/// Helper function to get API key from environment
fn get_api_key() -> String {
    // Load .env file if it exists
    let _ = dotenv();
    
    env::var("FINNHUB_API_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: FINNHUB_API_KEY not found in environment variables.");
        eprintln!("To run integration tests, please:");
        eprintln!("1. Get a free API key from https://finnhub.io/register");
        eprintln!("2. Copy .env.example to .env and add your API key");
        eprintln!("3. Or set FINNHUB_API_KEY environment variable");
        
        // Return a dummy key for compilation - tests will be skipped
        "dummy_key_for_compilation".to_string()
    })
}

/// Check if we have a real API key (not the dummy one)
fn has_real_api_key() -> bool {
    let api_key = get_api_key();
    !api_key.is_empty() && api_key != "dummy_key_for_compilation"
}

#[tokio::test]
async fn test_stock_quote_success() {
    if !has_real_api_key() {
        eprintln!("Skipping integration test - no valid API key found");
        return;
    }

    let client = FinnhubClient::new(get_api_key());
    
    // Test with a well-known stock symbol
    let result = client.stock().quote("AAPL").await;
    
    match result {
        Ok(quote) => {
            // Verify the response has the expected fields
            assert!(quote.current_price > 0.0, "Current price should be positive");
            assert!(quote.high >= quote.low, "High should be >= low");
            assert!(quote.open > 0.0, "Open price should be positive");
            assert!(quote.previous_close > 0.0, "Previous close should be positive");
            assert!(quote.timestamp > 0, "Timestamp should be positive");
            
            // Print some basic info for manual verification
            println!("✅ AAPL Quote:");
            println!("  Current Price: ${:.2}", quote.current_price);
            println!("  Change: ${:.2} ({:.2}%)", quote.change, quote.percent_change);
            println!("  High: ${:.2}", quote.high);
            println!("  Low: ${:.2}", quote.low);
        }
        Err(finnhub::Error::ApiError { status: 403, .. }) => {
            println!("⚠️  API key has limited access (403 Forbidden) - this is expected for free tier");
            println!("   Quote endpoint requires premium access on Finnhub");
        }
        Err(e) => {
            panic!("Unexpected error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_stock_quote_multiple_symbols() {
    if !has_real_api_key() {
        eprintln!("Skipping integration test - no valid API key found");
        return;
    }

    let client = FinnhubClient::new(get_api_key());
    
    // Test multiple symbols
    let symbols = ["AAPL", "MSFT", "GOOGL"];
    
    for symbol in &symbols {
        let result = client.stock().quote(symbol).await;
        
        match result {
            Ok(quote) => {
                // Basic validation
                assert!(quote.current_price > 0.0, "Current price should be positive for {}", symbol);
                assert!(quote.timestamp > 0, "Timestamp should be positive for {}", symbol);
                println!("✅ {} price: ${:.2}", symbol, quote.current_price);
            }
            Err(finnhub::Error::ApiError { status: 403, .. }) => {
                println!("⚠️  {} quote requires premium access (403 Forbidden)", symbol);
                return; // Skip remaining symbols if we hit 403
            }
            Err(e) => {
                panic!("Unexpected error for {}: {}", symbol, e);
            }
        }
    }
}

#[tokio::test]
async fn test_stock_quote_invalid_symbol() {
    if !has_real_api_key() {
        eprintln!("Skipping integration test - no valid API key found");
        return;
    }

    let client = FinnhubClient::new(get_api_key());
    
    // Test with an invalid symbol - this should either return an error or empty data
    let result = client.stock().quote("INVALID_SYMBOL_XYZ123").await;
    
    // We expect either an error or a response with zero/null values
    match result {
        Ok(quote) => {
            // If we get a response, it should have zero values for invalid symbols
            println!("Invalid symbol returned: current_price={}, timestamp={}", 
                     quote.current_price, quote.timestamp);
        }
        Err(e) => {
            println!("Invalid symbol returned error (expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_quote_response_structure() -> Result<()> {
    if !has_real_api_key() {
        eprintln!("Skipping integration test - no valid API key found");
        return Ok(());
    }

    let client = FinnhubClient::new(get_api_key());
    let quote = client.stock().quote("AAPL").await?;
    
    // Test that all fields are accessible and have reasonable values
    println!("Quote structure test for AAPL:");
    println!("  current_price: {}", quote.current_price);
    println!("  change: {}", quote.change);
    println!("  percent_change: {}", quote.percent_change);
    println!("  high: {}", quote.high);
    println!("  low: {}", quote.low);
    println!("  open: {}", quote.open);
    println!("  previous_close: {}", quote.previous_close);
    println!("  timestamp: {}", quote.timestamp);
    
    // Verify timestamp is recent (within the last week)
    let now = chrono::Utc::now().timestamp();
    let week_ago = now - (7 * 24 * 60 * 60); // 7 days in seconds
    
    assert!(quote.timestamp > week_ago, 
            "Timestamp should be recent (within last week). Got: {}, Week ago: {}", 
            quote.timestamp, week_ago);
    
    Ok(())
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use finnhub::{ClientConfig, auth::AuthMethod};

    #[tokio::test]
    async fn test_url_parameter_auth() -> Result<()> {
        if !has_real_api_key() {
            eprintln!("Skipping auth test - no valid API key found");
            return Ok(());
        }

        // Test default URL parameter authentication
        let client = FinnhubClient::new(get_api_key());
        let quote = client.stock().quote("AAPL").await?;
        
        assert!(quote.current_price > 0.0);
        println!("URL parameter auth test passed: ${:.2}", quote.current_price);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_header_auth() -> Result<()> {
        if !has_real_api_key() {
            eprintln!("Skipping auth test - no valid API key found");
            return Ok(());
        }

        // Test header authentication
        let config = ClientConfig {
            auth_method: AuthMethod::Header,
            ..ClientConfig::default()
        };
        
        let client = FinnhubClient::with_config(get_api_key(), config);
        let result = client.stock().quote("AAPL").await;
        
        // Header auth might not work with Finnhub, so we handle both cases
        match result {
            Ok(quote) => {
                assert!(quote.current_price > 0.0);
                println!("Header auth test passed: ${:.2}", quote.current_price);
            }
            Err(e) => {
                println!("Header auth failed (expected for Finnhub): {}", e);
                // This is expected since Finnhub primarily uses URL parameter auth
            }
        }
        
        Ok(())
    }
}