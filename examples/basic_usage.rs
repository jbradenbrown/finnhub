//! Basic usage example for the Finnhub client.

use finnhub::{FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with API key from environment variable
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY environment variable not set");
    
    let client = FinnhubClient::new(api_key);
    
    // Get a stock quote
    println!("Fetching AAPL quote...");
    let quote = client.stock().quote("AAPL").await?;
    println!("AAPL Quote:");
    println!("  Current Price: ${:.2}", quote.current_price);
    println!("  Change: ${:.2} ({:.2}%)", quote.change, quote.percent_change);
    println!("  High: ${:.2}", quote.high);
    println!("  Low: ${:.2}", quote.low);
    println!("  Open: ${:.2}", quote.open);
    println!("  Previous Close: ${:.2}", quote.previous_close);
    
    // Get company profile
    println!("\nFetching AAPL company profile...");
    let profile = client.stock().company_profile("AAPL").await?;
    println!("Company Profile:");
    println!("  Name: {}", profile.name);
    println!("  Country: {}", profile.country);
    println!("  Exchange: {}", profile.exchange);
    println!("  Industry: {}", profile.finnhub_industry);
    println!("  Market Cap: ${:.2}B", profile.market_capitalization / 1_000_000_000.0);
    
    // Get crypto exchanges
    println!("\nFetching crypto exchanges...");
    let exchanges = client.crypto().exchanges().await?;
    println!("Available crypto exchanges:");
    for exchange in exchanges.iter().take(5) {
        println!("  - {} ({})", exchange.name, exchange.code);
    }
    
    Ok(())
}