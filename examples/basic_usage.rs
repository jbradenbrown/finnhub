//! Basic usage example for the Finnhub client.

use chrono::{Duration, Utc};
use finnhub::{models::stock::CandleResolution, FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with API key from environment variable
    let api_key =
        std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY environment variable not set");

    let client = FinnhubClient::new(api_key);

    // Get a stock quote
    println!("Fetching AAPL quote...");
    let quote = client.stock().quote("AAPL").await?;
    println!("AAPL Quote:");
    println!("  Current Price: ${:.2}", quote.current_price);
    println!(
        "  Change: ${:.2} ({:.2}%)",
        quote.change, quote.percent_change
    );
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
    println!(
        "  Market Cap: ${:.2}B",
        profile.market_capitalization / 1_000_000_000.0
    );

    // Get daily candles for the last 30 days
    println!("\nFetching AAPL daily candles...");
    let to = Utc::now().timestamp();
    let from = (Utc::now() - Duration::days(30)).timestamp();
    let candles = client
        .stock()
        .candles("AAPL", CandleResolution::Daily, from, to)
        .await?;

    if candles.status == "ok" && !candles.close.is_empty() {
        println!("AAPL Daily Candles (last 5 days):");
        let len = candles.close.len();
        let start = if len > 5 { len - 5 } else { 0 };

        for i in start..len {
            println!(
                "  Date: {}, Close: ${:.2}",
                chrono::DateTime::from_timestamp(candles.timestamp[i], 0)
                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                candles.close[i]
            );
        }
    }

    // Get crypto exchanges
    println!("\nFetching crypto exchanges...");
    let exchanges = client.crypto().exchanges().await?;
    println!("Available crypto exchanges:");
    for exchange in exchanges.iter().take(5) {
        println!("  - {} ({})", exchange.name, exchange.code);
    }

    Ok(())
}
