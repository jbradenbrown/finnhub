//! Technical analysis example demonstrating scanner and technical indicator features.

use chrono::{Duration, Utc};
use finnhub::{FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY environment variable not set");

    let client = FinnhubClient::new(api_key);

    let symbol = "AAPL";
    println!("📊 Technical Analysis for {}", symbol);
    println!("{}", "=".repeat(50));

    // Get aggregate technical indicators
    analyze_aggregate_indicators(&client, symbol).await?;
    
    // Pattern recognition
    pattern_recognition(&client, symbol).await?;
    
    // Support and resistance levels
    support_resistance_levels(&client, symbol).await?;
    
    // Custom technical indicators
    custom_technical_indicators(&client, symbol).await?;
    
    // Multi-timeframe analysis
    multi_timeframe_analysis(&client, symbol).await?;

    Ok(())
}

async fn analyze_aggregate_indicators(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n📈 Aggregate Technical Indicators");
    println!("{}", "-".repeat(40));

    match client.scanner().aggregate_indicators(symbol, "D").await {
        Ok(indicators) => {
            let ta = &indicators.technical_analysis;
            let trend = &indicators.trend;
            
            println!("📊 Technical Analysis Summary:");
            println!("  Overall Signal: {}", ta.signal.to_uppercase());
            println!("  Buy Signals: {}", ta.count.buy);
            println!("  Neutral Signals: {}", ta.count.neutral);
            println!("  Sell Signals: {}", ta.count.sell);
            
            let total_signals = ta.count.buy + ta.count.neutral + ta.count.sell;
            if total_signals > 0 {
                let bullish_pct = (ta.count.buy as f64 / total_signals as f64) * 100.0;
                let bearish_pct = (ta.count.sell as f64 / total_signals as f64) * 100.0;
                println!("  Bullish: {:.1}% | Bearish: {:.1}%", bullish_pct, bearish_pct);
            }
            
            println!("\n📈 Trend Analysis:");
            println!("  ADX Value: {:.2}", trend.adx);
            println!("  Trending: {}", if trend.trending { "Yes" } else { "No" });
            
            let trend_strength = if trend.adx > 50.0 {
                "Very Strong"
            } else if trend.adx > 25.0 {
                "Strong"
            } else if trend.adx > 20.0 {
                "Moderate"
            } else {
                "Weak"
            };
            println!("  Trend Strength: {}", trend_strength);
        }
        Err(e) => println!("Aggregate indicators not available: {}", e),
    }

    Ok(())
}

async fn pattern_recognition(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n🔍 Pattern Recognition");
    println!("{}", "-".repeat(40));

    match client.scanner().pattern_recognition(symbol, "D").await {
        Ok(patterns) => {
            if patterns.points.is_empty() {
                println!("No chart patterns detected in recent data");
            } else {
                println!("📊 Detected Chart Patterns:");
                for (i, pattern) in patterns.points.iter().take(5).enumerate() {
                    let pattern_type_emoji = match pattern.patterntype.as_str() {
                        "bullish" => "📈",
                        "bearish" => "📉",
                        _ => "🔄",
                    };
                    
                    println!("  {}. {} {} ({})", 
                        i + 1, 
                        pattern_type_emoji, 
                        pattern.patternname, 
                        pattern.patterntype
                    );
                    
                    println!("     Status: {}", pattern.status);
                    println!("     Entry: {:.2}", pattern.entry);
                    println!("     Stop Loss: {:.2}", pattern.stoploss);
                    println!("     Target 1: {:.2}", pattern.profit1);
                    if pattern.profit2 != 0.0 {
                        println!("     Target 2: {:.2}", pattern.profit2);
                    }
                    
                    // Calculate risk/reward ratio
                    let risk = (pattern.entry - pattern.stoploss).abs();
                    let reward = (pattern.profit1 - pattern.entry).abs();
                    if risk > 0.0 {
                        let rr_ratio = reward / risk;
                        println!("     Risk/Reward: 1:{:.2}", rr_ratio);
                    }
                    println!();
                }
            }
        }
        Err(e) => println!("Pattern recognition not available: {}", e),
    }

    Ok(())
}

async fn support_resistance_levels(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n📏 Support & Resistance Levels");
    println!("{}", "-".repeat(40));

    match client.scanner().support_resistance(symbol, "D").await {
        Ok(levels) => {
            if levels.levels.is_empty() {
                println!("No support/resistance levels identified");
            } else {
                // Get current price for context
                let current_price = match client.stock().quote(symbol).await {
                    Ok(quote) => Some(quote.current_price),
                    Err(_) => None,
                };
                
                println!("📊 Key Support & Resistance Levels:");
                
                // Sort levels
                let mut sorted_levels = levels.levels.clone();
                sorted_levels.sort_by(|a, b| b.partial_cmp(a).unwrap());
                
                for (i, level) in sorted_levels.iter().take(8).enumerate() {
                    let level_type = if let Some(current) = current_price {
                        if *level > current {
                            "Resistance"
                        } else {
                            "Support"
                        }
                    } else {
                        "Level"
                    };
                    
                    let distance = if let Some(current) = current_price {
                        let dist_pct = ((*level - current) / current) * 100.0;
                        format!(" ({:+.1}%)", dist_pct)
                    } else {
                        String::new()
                    };
                    
                    println!("  {}. ${:.2} - {}{}", 
                        i + 1, level, level_type, distance);
                }
                
                if let Some(current) = current_price {
                    println!("\nCurrent Price: ${:.2}", current);
                    
                    // Find nearest support and resistance
                    let resistance_levels: Vec<f64> = sorted_levels.iter()
                        .filter(|&&level| level > current)
                        .cloned()
                        .collect();
                    let support_levels: Vec<f64> = sorted_levels.iter()
                        .filter(|&&level| level < current)
                        .cloned()
                        .collect();
                    
                    if let Some(&nearest_resistance) = resistance_levels.last() {
                        let resistance_dist = ((nearest_resistance - current) / current) * 100.0;
                        println!("Nearest Resistance: ${:.2} (+{:.1}%)", 
                            nearest_resistance, resistance_dist);
                    }
                    
                    if let Some(&nearest_support) = support_levels.first() {
                        let support_dist = ((current - nearest_support) / current) * 100.0;
                        println!("Nearest Support: ${:.2} (-{:.1}%)", 
                            nearest_support, support_dist);
                    }
                }
            }
        }
        Err(e) => println!("Support/resistance levels not available: {}", e),
    }

    Ok(())
}

async fn custom_technical_indicators(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n🔧 Custom Technical Indicators");
    println!("{}", "-".repeat(40));

    let to = Utc::now().timestamp();
    let from = (Utc::now() - Duration::days(50)).timestamp();

    // RSI (Relative Strength Index)
    match client.misc().technical_indicator(symbol, "D", from, to, "rsi", Some(serde_json::json!({"timeperiod": 14}))).await {
        Ok(rsi_data) => {
            if let Some(rsi_values) = rsi_data.indicators.get("rsi") {
                if let Some(&latest_rsi) = rsi_values.last() {
                    println!("📈 RSI (14): {:.2}", latest_rsi);
                    
                    let rsi_signal = if latest_rsi > 70.0 {
                        "Overbought 🔴"
                    } else if latest_rsi < 30.0 {
                        "Oversold 🟢"
                    } else if latest_rsi > 50.0 {
                        "Bullish 📈"
                    } else {
                        "Bearish 📉"
                    };
                    println!("  Signal: {}", rsi_signal);
                }
            }
        }
        Err(e) => println!("RSI not available: {}", e),
    }

    // MACD
    match client.misc().technical_indicator(symbol, "D", from, to, "macd", None).await {
        Ok(macd_data) => {
            if let (Some(macd_line), Some(signal_line)) = 
                (macd_data.indicators.get("macd"), macd_data.indicators.get("macdsignal")) {
                if let (Some(&latest_macd), Some(&latest_signal)) = 
                    (macd_line.last(), signal_line.last()) {
                    println!("📊 MACD: {:.3}", latest_macd);
                    println!("  Signal Line: {:.3}", latest_signal);
                    
                    let macd_signal = if latest_macd > latest_signal {
                        "Bullish Crossover 📈"
                    } else {
                        "Bearish Crossover 📉"
                    };
                    println!("  Status: {}", macd_signal);
                }
            }
        }
        Err(e) => println!("MACD not available: {}", e),
    }

    // Bollinger Bands
    match client.misc().technical_indicator(symbol, "D", from, to, "bbands", Some(serde_json::json!({"timeperiod": 20, "nbdevup": 2, "nbdevdn": 2}))).await {
        Ok(bb_data) => {
            if let (Some(upper), Some(middle), Some(lower)) = 
                (bb_data.indicators.get("upperband"), 
                 bb_data.indicators.get("middleband"), 
                 bb_data.indicators.get("lowerband")) {
                if let (Some(&bb_upper), Some(&bb_middle), Some(&bb_lower)) = 
                    (upper.last(), middle.last(), lower.last()) {
                    
                    println!("📏 Bollinger Bands (20,2):");
                    println!("  Upper: ${:.2}", bb_upper);
                    println!("  Middle: ${:.2}", bb_middle);
                    println!("  Lower: ${:.2}", bb_lower);
                    
                    // Get current price for position analysis
                    if let Ok(quote) = client.stock().quote(symbol).await {
                        let current = quote.current_price;
                        let bb_position = if current > bb_upper {
                            "Above Upper Band (Overbought)"
                        } else if current < bb_lower {
                            "Below Lower Band (Oversold)"
                        } else {
                            "Within Bands (Normal)"
                        };
                        println!("  Position: {}", bb_position);
                        
                        let bb_width = ((bb_upper - bb_lower) / bb_middle) * 100.0;
                        println!("  Band Width: {:.2}% (Volatility)", bb_width);
                    }
                }
            }
        }
        Err(e) => println!("Bollinger Bands not available: {}", e),
    }

    Ok(())
}

async fn multi_timeframe_analysis(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n⏰ Multi-Timeframe Analysis");
    println!("{}", "-".repeat(40));

    let timeframes = [
        ("Daily", "D"),
        ("4-Hour", "240"),
        ("1-Hour", "60"),
    ];

    for (name, resolution) in &timeframes {
        println!("\n📊 {} Timeframe:", name);
        
        match client.scanner().aggregate_indicators(symbol, resolution).await {
            Ok(indicators) => {
                let signal = &indicators.technical_analysis.signal;
                let adx = indicators.trend.adx;
                
                let signal_emoji = match signal.as_str() {
                    "buy" => "🟢",
                    "sell" => "🔴",
                    _ => "🟡",
                };
                
                println!("  Signal: {} {} | ADX: {:.1}", signal_emoji, signal.to_uppercase(), adx);
                
                // Calculate signal strength
                let total = indicators.technical_analysis.count.buy + 
                           indicators.technical_analysis.count.neutral + 
                           indicators.technical_analysis.count.sell;
                           
                if total > 0 {
                    let buy_strength = (indicators.technical_analysis.count.buy as f64 / total as f64) * 100.0;
                    let sell_strength = (indicators.technical_analysis.count.sell as f64 / total as f64) * 100.0;
                    println!("  Strength: {:.0}% Buy | {:.0}% Sell", buy_strength, sell_strength);
                }
            }
            Err(e) => println!("  {} timeframe not available: {}", name, e),
        }
    }

    // Trend alignment analysis
    println!("\n🎯 Trend Alignment Summary:");
    println!("Look for confluence across timeframes for stronger signals.");
    println!("Best setups occur when multiple timeframes align in the same direction.");

    Ok(())
}