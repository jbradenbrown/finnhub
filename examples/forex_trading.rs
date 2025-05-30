//! Forex trading example demonstrating currency market data and analysis.

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

    println!("💱 Forex Market Analysis");
    println!("{}", "=".repeat(50));

    // Major currency pairs to analyze
    let major_pairs = vec!["EUR/USD", "GBP/USD", "USD/JPY", "USD/CHF", "AUD/USD", "USD/CAD"];

    // Get forex exchanges
    get_forex_exchanges(&client).await?;
    
    // Analyze major currency pairs
    analyze_currency_pairs(&client, &major_pairs).await?;
    
    // Get current exchange rates
    get_exchange_rates(&client).await?;
    
    // Technical analysis for EUR/USD
    technical_analysis(&client, "OANDA:EUR_USD").await?;
    
    // Cross-currency analysis
    cross_currency_analysis(&client).await?;

    Ok(())
}

async fn get_forex_exchanges(client: &FinnhubClient) -> Result<()> {
    println!("\n🏛️ Available Forex Exchanges");
    println!("{}", "-".repeat(40));

    match client.forex().exchanges().await {
        Ok(exchanges) => {
            println!("Found {} forex exchanges:", exchanges.len());
            for (i, exchange) in exchanges.iter().take(5).enumerate() {
                println!("  {}. {}", i + 1, exchange);
            }
            if exchanges.len() > 5 {
                println!("  ... and {} more exchanges", exchanges.len() - 5);
            }
        }
        Err(e) => println!("Forex exchanges not available: {}", e),
    }

    Ok(())
}

async fn analyze_currency_pairs(client: &FinnhubClient, pairs: &[&str]) -> Result<()> {
    println!("\n📊 Major Currency Pairs Analysis");
    println!("{}", "-".repeat(40));

    for pair in pairs {
        // Convert pair format for API (EUR/USD -> OANDA:EUR_USD)
        let symbol = format!("OANDA:{}", pair.replace('/', "_"));
        
        match client.forex().candles(&symbol, CandleResolution::Daily, 
            (Utc::now() - Duration::days(7)).timestamp(), 
            Utc::now().timestamp()).await {
            Ok(candles) => {
                if candles.status == "ok" && !candles.close.is_empty() {
                    let current = candles.close[candles.close.len() - 1];
                    let previous = if candles.close.len() > 1 {
                        candles.close[candles.close.len() - 2]
                    } else {
                        current
                    };
                    
                    let change = current - previous;
                    let change_pct = (change / previous) * 100.0;
                    
                    let trend = if change > 0.0 { "📈" } else { "📉" };
                    
                    println!("{} {}: {:.5} {} {:.5} ({:.3}%)", 
                        trend, pair, current, 
                        if change > 0.0 { "+" } else { "" }, 
                        change, change_pct);
                    
                    // Calculate daily range
                    let high = candles.high[candles.high.len() - 1];
                    let low = candles.low[candles.low.len() - 1];
                    let range_pct = ((high - low) / current) * 100.0;
                    println!("    Range: {:.5} - {:.5} ({:.3}%)", low, high, range_pct);
                }
            }
            Err(e) => println!("{}: Data not available ({})", pair, e),
        }
    }

    Ok(())
}

async fn get_exchange_rates(client: &FinnhubClient) -> Result<()> {
    println!("\n💰 Current Exchange Rates (USD Base)");
    println!("{}", "-".repeat(40));

    match client.forex().rates("USD").await {
        Ok(rates) => {
            println!("Base Currency: USD");
            println!("Exchange rates:");
            
            // Group currencies by region/type
            let major_currencies = ["EUR", "GBP", "JPY", "CHF", "CAD", "AUD"];
            let emerging_currencies = ["CNY", "INR", "BRL", "MXN", "KRW"];
            
            println!("\n  📍 Major Currencies:");
            for currency in &major_currencies {
                if let Some(rate) = rates.quote.get(*currency) {
                    println!("    USD/{}: {:.4}", currency, rate);
                }
            }
            
            println!("\n  🌏 Emerging Market Currencies:");
            for currency in &emerging_currencies {
                if let Some(rate) = rates.quote.get(*currency) {
                    println!("    USD/{}: {:.4}", currency, rate);
                }
            }
            
            // Show crypto rates if available
            let crypto_currencies = ["BTC", "ETH"];
            let mut has_crypto = false;
            for currency in &crypto_currencies {
                if rates.quote.contains_key(*currency) {
                    if !has_crypto {
                        println!("\n  ₿ Cryptocurrency Rates:");
                        has_crypto = true;
                    }
                    if let Some(rate) = rates.quote.get(*currency) {
                        println!("    USD/{}: {:.2}", currency, rate);
                    }
                }
            }
        }
        Err(e) => println!("Exchange rates not available: {}", e),
    }

    Ok(())
}

async fn technical_analysis(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n📈 Technical Analysis: EUR/USD");
    println!("{}", "-".repeat(40));

    let to = Utc::now().timestamp();
    let from = (Utc::now() - Duration::days(30)).timestamp();
    
    match client.forex().candles(symbol, CandleResolution::Daily, from, to).await {
        Ok(candles) => {
            if candles.status == "ok" && !candles.close.is_empty() {
                let prices = &candles.close;
                let highs = &candles.high;
                let lows = &candles.low;
                
                // Calculate moving averages
                let ma_5 = if prices.len() >= 5 {
                    prices.iter().rev().take(5).sum::<f64>() / 5.0
                } else {
                    prices.iter().sum::<f64>() / prices.len() as f64
                };
                
                let ma_20 = if prices.len() >= 20 {
                    prices.iter().rev().take(20).sum::<f64>() / 20.0
                } else {
                    prices.iter().sum::<f64>() / prices.len() as f64
                };
                
                let current = prices[prices.len() - 1];
                
                println!("Current Rate: {:.5}", current);
                println!("5-day MA: {:.5}", ma_5);
                println!("20-day MA: {:.5}", ma_20);
                
                // Determine trend
                let short_term_trend = if current > ma_5 { "Bullish" } else { "Bearish" };
                let long_term_trend = if ma_5 > ma_20 { "Bullish" } else { "Bearish" };
                
                println!("Short-term Trend: {}", short_term_trend);
                println!("Long-term Trend: {}", long_term_trend);
                
                // Calculate support and resistance levels
                let period_high = highs.iter().fold(0.0f64, |acc, &x| acc.max(x));
                let period_low = lows.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));
                
                println!("30-day High: {:.5}", period_high);
                println!("30-day Low: {:.5}", period_low);
                
                // Calculate ATR (Average True Range) for volatility
                let mut true_ranges = Vec::new();
                for i in 1..prices.len() {
                    let tr1 = highs[i] - lows[i];
                    let tr2 = (highs[i] - prices[i-1]).abs();
                    let tr3 = (lows[i] - prices[i-1]).abs();
                    true_ranges.push(tr1.max(tr2).max(tr3));
                }
                
                if !true_ranges.is_empty() {
                    let atr = true_ranges.iter().sum::<f64>() / true_ranges.len() as f64;
                    println!("Average True Range: {:.5}", atr);
                    println!("Volatility: {:.3}%", (atr / current) * 100.0);
                }
                
                // Price position analysis
                let position_in_range = (current - period_low) / (period_high - period_low);
                println!("Position in 30-day range: {:.1}%", position_in_range * 100.0);
            }
        }
        Err(e) => println!("Technical analysis data not available: {}", e),
    }

    Ok(())
}

async fn cross_currency_analysis(client: &FinnhubClient) -> Result<()> {
    println!("\n🔄 Cross-Currency Analysis");
    println!("{}", "-".repeat(40));

    // Calculate implied cross rates
    match client.forex().rates("USD").await {
        Ok(usd_rates) => {
            println!("📊 Implied Cross Rates:");
            
            // EUR-based crosses
            if let (Some(eur_usd), Some(gbp_usd)) = 
                (usd_rates.quote.get("EUR"), usd_rates.quote.get("GBP")) {
                let eur_gbp = eur_usd / gbp_usd;
                println!("  EUR/GBP: {:.5}", eur_gbp);
            }
            
            if let (Some(eur_usd), Some(jpy_usd)) = 
                (usd_rates.quote.get("EUR"), usd_rates.quote.get("JPY")) {
                let eur_jpy = eur_usd * jpy_usd;
                println!("  EUR/JPY: {:.3}", eur_jpy);
            }
            
            // GBP-based crosses
            if let (Some(gbp_usd), Some(jpy_usd)) = 
                (usd_rates.quote.get("GBP"), usd_rates.quote.get("JPY")) {
                let gbp_jpy = gbp_usd * jpy_usd;
                println!("  GBP/JPY: {:.3}", gbp_jpy);
            }
            
            // Commodity currency analysis
            println!("\n💎 Commodity Currencies vs USD:");
            let commodity_currencies = [
                ("AUD", "Australian Dollar"),
                ("CAD", "Canadian Dollar"),
                ("NZD", "New Zealand Dollar"),
            ];
            
            for (code, name) in &commodity_currencies {
                if let Some(rate) = usd_rates.quote.get(*code) {
                    println!("  {} ({}): {:.4}", code, name, rate);
                }
            }
            
            // Calculate DXY-style index (simplified)
            let weights = [
                ("EUR", 0.576),
                ("JPY", 0.136),
                ("GBP", 0.119),
                ("CAD", 0.091),
                ("CHF", 0.036),
                ("SEK", 0.042),
            ];
            
            let mut index_value = 0.0;
            let mut total_weight = 0.0;
            
            for (currency, weight) in &weights {
                if let Some(rate) = usd_rates.quote.get(*currency) {
                    // For USD index, we want 1/rate for non-USD currencies
                    index_value += weight * (1.0 / rate);
                    total_weight += weight;
                }
            }
            
            if total_weight > 0.0 {
                let normalized_index = (index_value / total_weight) * 100.0;
                println!("\n📈 USD Strength Index (simplified): {:.2}", normalized_index);
            }
        }
        Err(e) => println!("Cross-currency analysis not available: {}", e),
    }

    Ok(())
}