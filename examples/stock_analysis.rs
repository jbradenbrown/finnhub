//! Stock analysis example demonstrating fundamental and technical analysis features.

use chrono::{Duration, Utc};
use finnhub::{
    models::stock::CandleResolution,
    FinnhubClient, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY environment variable not set");

    let client = FinnhubClient::new(api_key);

    let symbol = "AAPL";
    println!("🍎 Comprehensive Stock Analysis for {}", symbol);
    println!("{}", "=".repeat(60));

    // Basic quote and profile
    analyze_basic_info(&client, symbol).await?;
    
    // Technical analysis
    analyze_technical_data(&client, symbol).await?;
    
    // Fundamental analysis
    analyze_fundamentals(&client, symbol).await?;
    
    // Market sentiment
    analyze_sentiment(&client, symbol).await?;
    
    // Insider activity
    analyze_insider_activity(&client, symbol).await?;

    Ok(())
}

async fn analyze_basic_info(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n📊 Basic Information");
    println!("{}", "-".repeat(40));

    // Get current quote
    let quote = client.stock().quote(symbol).await?;
    println!("Current Price: ${:.2}", quote.current_price);
    println!("Change: ${:.2} ({:.2}%)", quote.change, quote.percent_change);
    println!("Day Range: ${:.2} - ${:.2}", quote.low, quote.high);
    println!("Previous Close: ${:.2}", quote.previous_close);

    // Get company profile
    match client.stock().company_profile(symbol).await {
        Ok(profile) => {
            println!("Company: {}", profile.name.unwrap_or_else(|| "N/A".to_string()));
            if let Some(market_cap) = profile.market_capitalization {
                println!("Market Cap: ${:.2}B", market_cap / 1_000_000_000.0);
            }
            if let Some(shares) = profile.share_outstanding {
                println!("Shares Outstanding: {:.2}M", shares / 1_000_000.0);
            }
            println!("Industry: {}", profile.finnhub_industry.unwrap_or_else(|| "N/A".to_string()));
            println!("Exchange: {}", profile.exchange.unwrap_or_else(|| "N/A".to_string()));
        }
        Err(e) => println!("Company profile not available: {}", e),
    }

    Ok(())
}

async fn analyze_technical_data(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n📈 Technical Analysis");
    println!("{}", "-".repeat(40));

    // Get candle data for last 30 days
    let to = Utc::now().timestamp();
    let from = (Utc::now() - Duration::days(30)).timestamp();
    
    match client.stock().candles(symbol, CandleResolution::Daily, from, to).await {
        Ok(candles) => {
            if candles.status == "ok" && !candles.close.is_empty() {
                let prices = &candles.close;
                let current = prices[prices.len() - 1];
                let ma_5 = prices.iter().rev().take(5).sum::<f64>() / 5.0;
                let ma_20 = if prices.len() >= 20 {
                    prices.iter().rev().take(20).sum::<f64>() / 20.0
                } else {
                    prices.iter().sum::<f64>() / prices.len() as f64
                };

                println!("Current Price: ${:.2}", current);
                println!("5-day MA: ${:.2}", ma_5);
                println!("20-day MA: ${:.2}", ma_20);
                
                let trend = if current > ma_5 && ma_5 > ma_20 {
                    "📈 Bullish"
                } else if current < ma_5 && ma_5 < ma_20 {
                    "📉 Bearish"
                } else {
                    "🔄 Sideways"
                };
                println!("Short-term Trend: {}", trend);

                // Calculate volatility (standard deviation)
                let mean = prices.iter().sum::<f64>() / prices.len() as f64;
                let variance = prices.iter()
                    .map(|price| (price - mean).powi(2))
                    .sum::<f64>() / prices.len() as f64;
                let volatility = variance.sqrt();
                println!("30-day Volatility: {:.2}%", (volatility / mean) * 100.0);
            }
        }
        Err(e) => println!("Candle data not available: {}", e),
    }

    // Get bid-ask spread
    match client.stock().bid_ask(symbol).await {
        Ok(bid_ask) => {
            if let (Some(bid), Some(ask)) = (bid_ask.bid, bid_ask.ask) {
                let spread = ask - bid;
                let spread_pct = (spread / ((bid + ask) / 2.0)) * 100.0;
                println!("Bid-Ask Spread: ${:.3} ({:.3}%)", spread, spread_pct);
            }
        }
        Err(e) => println!("Bid-ask data not available: {}", e),
    }

    Ok(())
}

async fn analyze_fundamentals(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n💰 Fundamental Analysis");
    println!("{}", "-".repeat(40));

    // Get basic financials
    match client.stock().metrics(symbol).await {
        Ok(metrics) => {
            println!("📊 Key Metrics:");
            println!("  Found {} financial metrics", metrics.metric.len());
        }
        Err(e) => println!("Metrics not available: {}", e),
    }

    // Get earnings estimates
    match client.stock().eps_estimates(symbol, None).await {
        Ok(estimates) => {
            println!("📈 Earnings Estimates:");
            for estimate in estimates.data.iter().take(2) {
                println!("  {} - EPS Est: ${:.2}", 
                    estimate.period, 
                    estimate.eps_avg.unwrap_or(0.0)
                );
            }
        }
        Err(e) => println!("EPS estimates not available: {}", e),
    }

    // Get price targets
    match client.stock().price_target(symbol).await {
        Ok(target) => {
            println!("🎯 Analyst Price Targets:");
            println!("  Mean Target: ${:.2}", target.target_mean);
            println!("  High Target: ${:.2}", target.target_high);
            println!("  Low Target: ${:.2}", target.target_low);
        }
        Err(e) => println!("Price targets not available: {}", e),
    }

    Ok(())
}

async fn analyze_sentiment(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n🎭 Market Sentiment");
    println!("{}", "-".repeat(40));

    // Get recommendations
    match client.stock().recommendations(symbol).await {
        Ok(recommendations) => {
            if let Some(latest) = recommendations.first() {
                println!("📊 Analyst Recommendations ({})", latest.period);
                println!("  Strong Buy: {}", latest.strong_buy);
                println!("  Buy: {}", latest.buy);
                println!("  Hold: {}", latest.hold);
                println!("  Sell: {}", latest.sell);
                println!("  Strong Sell: {}", latest.strong_sell);
                
                let total = latest.strong_buy + latest.buy + latest.hold + latest.sell + latest.strong_sell;
                let bullish_pct = ((latest.strong_buy + latest.buy) as f64 / total as f64) * 100.0;
                println!("  Bullish Sentiment: {:.1}%", bullish_pct);
            }
        }
        Err(e) => println!("Recommendations not available: {}", e),
    }

    // Get social sentiment
    match client.stock().social_sentiment(symbol, "2024-01-01", "2024-12-31").await {
        Ok(sentiment) => {
            if let Some(reddit) = sentiment.reddit.as_ref().and_then(|r| r.first()) {
                println!("🗣️ Social Sentiment (Reddit):");
                println!("  Score: {:.2}", reddit.score);
                println!("  Mentions: {}", reddit.mention);
                println!("  Positive Mention: {}", reddit.positive_mention);
                println!("  Negative Mention: {}", reddit.negative_mention);
            }
        }
        Err(e) => println!("Social sentiment not available: {}", e),
    }

    Ok(())
}

async fn analyze_insider_activity(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n👥 Insider Activity");
    println!("{}", "-".repeat(40));

    // Get insider transactions
    match client.stock().insider_transactions(symbol).await {
        Ok(transactions) => {
            println!("💼 Recent Insider Transactions:");
            for transaction in transactions.data.iter().take(3) {
                let change_val = transaction.change.unwrap_or(0);
                let transaction_type = if change_val > 0 { "BUY" } else { "SELL" };
                println!("  {} - {} {} shares by {}", 
                    transaction.filing_date,
                    transaction_type,
                    change_val.abs(),
                    transaction.name
                );
            }
        }
        Err(e) => println!("Insider transactions not available: {}", e),
    }

    // Get insider sentiment
    match client.stock().insider_sentiment(symbol, "2024-01-01", "2024-12-31").await {
        Ok(sentiment) => {
            if let Some(data) = sentiment.data.first() {
                println!("📊 Insider Sentiment Summary:");
                println!("  Period: {}-{}", data.year, data.month);
                println!("  Net Change: {}", data.change);
                println!("  Monthly Share Purchase Ratio: {:.2}", data.mspr);
            }
        }
        Err(e) => println!("Insider sentiment not available: {}", e),
    }

    Ok(())
}