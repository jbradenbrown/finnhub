//! Basic usage example for the Finnhub client.

use chrono::{Duration, Utc};
use finnhub::{
    models::{
        news::NewsCategory,
        stock::{CandleResolution, StatementFrequency, StatementType},
    },
    FinnhubClient, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with API key from environment variable
    let api_key =
        std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY environment variable not set");

    // Create client with default URL parameter authentication (recommended by Finnhub)
    let client = FinnhubClient::new(api_key);

    // Alternative: Create client with custom configuration
    // use finnhub::{ClientConfig, auth::AuthMethod};
    // let config = ClientConfig {
    //     auth_method: AuthMethod::Header, // Use header authentication instead
    //     ..ClientConfig::default()
    // };
    // let client = FinnhubClient::with_config(api_key, config);

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

    // Get price target
    println!("\nFetching AAPL price target...");
    match client.stock().price_target("AAPL").await {
        Ok(target) => {
            println!("Price Target:");
            println!("  Mean: ${:.2}", target.target_mean);
            println!("  High: ${:.2}", target.target_high);
            println!("  Low: ${:.2}", target.target_low);
            println!("  Analysts: {}", target.number_analysts);
        }
        Err(e) => println!("Price target not available: {}", e),
    }

    // Get recommendations
    println!("\nFetching AAPL recommendations...");
    match client.stock().recommendations("AAPL").await {
        Ok(recommendations) => {
            if let Some(latest) = recommendations.first() {
                println!("Latest Recommendations:");
                println!("  Strong Buy: {}", latest.strong_buy);
                println!("  Buy: {}", latest.buy);
                println!("  Hold: {}", latest.hold);
                println!("  Sell: {}", latest.sell);
                println!("  Strong Sell: {}", latest.strong_sell);
                println!("  Period: {}", latest.period);
            }
        }
        Err(e) => println!("Recommendations not available: {}", e),
    }

    // Get financials (income statement)
    println!("\nFetching AAPL annual income statement...");
    match client
        .stock()
        .financials(
            "AAPL",
            StatementType::IncomeStatement,
            StatementFrequency::Annual,
        )
        .await
    {
        Ok(financials) => {
            println!(
                "Financial data available for {} periods",
                financials.financials.len()
            );
        }
        Err(e) => println!("Financials not available: {}", e),
    }

    // Get historical market cap
    println!("\nFetching AAPL historical market cap...");
    let to_date = Utc::now().format("%Y-%m-%d").to_string();
    let from_date = (Utc::now() - Duration::days(90))
        .format("%Y-%m-%d")
        .to_string();

    match client
        .stock()
        .historical_market_cap("AAPL", &from_date, &to_date)
        .await
    {
        Ok(historical_data) => {
            println!("Historical Market Cap ({})", historical_data.currency);
            for (i, data_point) in historical_data.data.iter().rev().take(5).enumerate() {
                println!(
                    "  {}: ${:.2}B",
                    data_point.at_date,
                    data_point.market_capitalization / 1_000.0
                );
            }
        }
        Err(e) => println!("Historical market cap not available: {}", e),
    }

    // Get company peers
    println!("\nFetching AAPL peers...");
    match client.stock().peers("AAPL", None).await {
        Ok(peers) => {
            println!("Company Peers:");
            for peer in peers.iter().take(5) {
                println!("  - {}", peer);
            }
        }
        Err(e) => println!("Peers not available: {}", e),
    }

    // Get market status
    println!("\nChecking US market status...");
    match client.stock().market_status("US").await {
        Ok(status) => {
            println!("Market Status:");
            println!("  Exchange: {}", status.exchange);
            println!("  Is Open: {}", status.is_open);
            if let Some(session) = status.session {
                println!("  Session: {}", session);
            }
            println!("  Timezone: {}", status.timezone);
        }
        Err(e) => println!("Market status not available: {}", e),
    }

    // Get crypto exchanges
    println!("\nFetching crypto exchanges...");
    let exchanges = client.crypto().exchanges().await?;
    println!("Available crypto exchanges:");
    for exchange in exchanges.iter().take(5) {
        println!("  - {} ({})", exchange.name, exchange.code);
    }

    // Get market news
    println!("\nFetching general market news...");
    match client.news().market_news(NewsCategory::General, None).await {
        Ok(news) => {
            println!("Latest market news:");
            for article in news.iter().take(3) {
                println!("  📰 {}", article.headline);
                println!(
                    "     Source: {} | {}",
                    article.source,
                    chrono::DateTime::from_timestamp(article.datetime, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "Unknown".to_string())
                );
            }
        }
        Err(e) => println!("News not available: {}", e),
    }

    // Test forex rates (if available)
    println!("\nFetching USD forex rates...");
    match client.forex().rates("USD").await {
        Ok(rates) => {
            println!("USD Exchange Rates:");
            for (currency, rate) in rates.quote.iter().take(5) {
                println!("  USD/{}: {:.4}", currency, rate);
            }
        }
        Err(e) => println!("Forex rates not available: {}", e),
    }

    Ok(())
}
