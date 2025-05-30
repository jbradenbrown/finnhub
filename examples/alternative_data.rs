//! Alternative data example demonstrating ESG, patent, insider, and other non-traditional data sources.

use chrono::Datelike;
use finnhub::{FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY environment variable not set");

    let client = FinnhubClient::new(api_key);

    let symbol = "AAPL";
    println!("🔍 Alternative Data Analysis for {}", symbol);
    println!("{}", "=".repeat(55));

    // ESG Analysis
    esg_analysis(&client, symbol).await?;
    
    // Patent and Innovation Data
    patent_analysis(&client, symbol).await?;
    
    // Miscellaneous Alternative Data
    misc_alternative_data(&client).await?;

    Ok(())
}

async fn esg_analysis(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n🌱 ESG (Environmental, Social, Governance) Analysis");
    println!("{}", "-".repeat(50));

    match client.stock().esg(symbol).await {
        Ok(esg) => {
            println!("🏢 Company: {}", esg.company_name.as_deref().unwrap_or(symbol));
            
            if let Some(overall_rating) = esg.esg_risk_rating {
                println!("📊 Overall ESG Risk Rating: {:.1}", overall_rating);
                
                if let Some(level) = &esg.esg_risk_level {
                    let risk_emoji = match level.as_str() {
                        "Low" => "🟢",
                        "Medium" => "🟡",
                        "High" => "🔴",
                        _ => "⚪",
                    };
                    println!("🎯 Risk Level: {} {}", risk_emoji, level);
                }
            }
            
            println!("\n📈 Component Scores:");
            
            if let Some(env_score) = esg.environment_risk_score {
                let env_rating = score_to_rating(env_score);
                println!("  🌍 Environmental: {:.1} ({})", env_score, env_rating);
            }
            
            if let Some(social_score) = esg.social_risk_score {
                let social_rating = score_to_rating(social_score);
                println!("  👥 Social: {:.1} ({})", social_score, social_rating);
            }
            
            if let Some(gov_score) = esg.governance_risk_score {
                let gov_rating = score_to_rating(gov_score);
                println!("  🏛️ Governance: {:.1} ({})", gov_score, gov_rating);
            }
            
            // ESG interpretation
            if let Some(overall) = esg.esg_risk_rating {
                println!("\n💡 ESG Insights:");
                if overall < 10.0 {
                    println!("  ✅ Negligible ESG risk - Strong sustainability practices");
                } else if overall < 20.0 {
                    println!("  🟢 Low ESG risk - Good sustainability management");
                } else if overall < 30.0 {
                    println!("  🟡 Medium ESG risk - Some areas for improvement");
                } else if overall < 40.0 {
                    println!("  🔶 High ESG risk - Significant sustainability concerns");
                } else {
                    println!("  🔴 Severe ESG risk - Major sustainability issues");
                }
            }
        }
        Err(e) => println!("ESG data not available: {}", e),
    }

    Ok(())
}

fn score_to_rating(score: f64) -> &'static str {
    if score < 10.0 {
        "Excellent"
    } else if score < 20.0 {
        "Good"
    } else if score < 30.0 {
        "Average"
    } else if score < 40.0 {
        "Poor"
    } else {
        "Critical"
    }
}

async fn patent_analysis(client: &FinnhubClient, symbol: &str) -> Result<()> {
    println!("\n💡 Innovation & Patent Analysis");
    println!("{}", "-".repeat(50));

    let current_year = chrono::Utc::now().year();
    let last_year = current_year - 1;
    
    let from_date = format!("{}-01-01", last_year);
    let to_date = format!("{}-12-31", current_year);

    match client.stock().uspto_patents(symbol, &from_date, &to_date).await {
        Ok(patents) => {
            if patents.data.is_empty() {
                println!("No patent applications found for the specified period");
            } else {
                println!("📋 Patent Applications ({} - {}):", last_year, current_year);
                println!("Total Applications: {}", patents.data.len());
                
                println!("\n📈 Innovation Metrics:");
                println!("  {} total applications in period", patents.data.len());
                let avg_per_year = patents.data.len() as f64 / 2.0; // 2-year period
                println!("  ~{:.1} applications per year (average)", avg_per_year);
            }
        }
        Err(e) => println!("Patent data not available: {}", e),
    }

    Ok(())
}

async fn misc_alternative_data(client: &FinnhubClient) -> Result<()> {
    println!("\n🌐 Miscellaneous Alternative Data");
    println!("{}", "-".repeat(50));

    // COVID-19 data
    match client.misc().covid19().await {
        Ok(covid_data) => {
            println!("🦠 COVID-19 Data (US States):");
            
            // Find top 5 states by cases
            let mut sorted_indices: Vec<usize> = (0..covid_data.len()).collect();
            sorted_indices.sort_by(|&a, &b| covid_data[b].cases.partial_cmp(&covid_data[a].cases).unwrap());
            
            for (i, &idx) in sorted_indices.iter().take(5).enumerate() {
                let state_data = &covid_data[idx];
                println!("  {}. {}: {:.0} cases, {:.0} deaths", 
                    i + 1, state_data.state, state_data.cases, state_data.death);
            }
            
            let total_cases: f64 = covid_data.iter().map(|s| s.cases).sum();
            let total_deaths: f64 = covid_data.iter().map(|s| s.death).sum();
            println!("  Total US: {:.0} cases, {:.0} deaths", total_cases, total_deaths);
        }
        Err(e) => println!("COVID-19 data not available: {}", e),
    }

    // Country metadata and risk factors
    match client.misc().country().await {
        Ok(countries) => {
            println!("\n🗺️ Country Risk Analysis (Sample):");
            
            // Filter for major economies and sort by risk
            let mut major_economies: Vec<_> = countries.iter()
                .filter(|c| ["US", "CN", "JP", "DE", "GB", "FR", "IN", "IT", "BR", "CA"]
                    .contains(&c.code2.as_str()))
                .collect();
            
            major_economies.sort_by(|a, b| 
                a.country_risk_premium.unwrap_or(0.0)
                    .partial_cmp(&b.country_risk_premium.unwrap_or(0.0))
                    .unwrap());
            
            for country in major_economies.iter().take(5) {
                let risk_premium = country.country_risk_premium.unwrap_or(0.0);
                let equity_premium = country.equity_risk_premium.unwrap_or(0.0);
                
                println!("  {} ({}): Risk {:.2}%, Equity Premium {:.2}%", 
                    country.code2, 
                    country.rating.as_ref().unwrap_or(&"N/A".to_string()),
                    risk_premium,
                    equity_premium
                );
            }
        }
        Err(e) => println!("Country data not available: {}", e),
    }

    // FDA calendar for healthcare/pharma insights
    match client.misc().fda_calendar().await {
        Ok(fda_events) => {
            println!("\n💊 Upcoming FDA Events:");
            
            if fda_events.is_empty() {
                println!("No upcoming FDA events scheduled");
            } else {
                for (i, event) in fda_events.iter().take(3).enumerate() {
                    println!("  {}. {} to {}", 
                        i + 1, event.from_date, event.to_date);
                    
                    // Truncate long descriptions
                    let description = if event.event_description.len() > 100 {
                        format!("{}...", &event.event_description[..97])
                    } else {
                        event.event_description.clone()
                    };
                    println!("     {}", description);
                }
            }
        }
        Err(e) => println!("FDA calendar not available: {}", e),
    }

    // Symbol search example
    match client.misc().symbol_search("tesla", Some("US")).await {
        Ok(results) => {
            println!("\n🔍 Symbol Search Example (Tesla):");
            println!("Found {} results:", results.count);
            for (i, result) in results.result.iter().take(3).enumerate() {
                println!("  {}. {} - {} ({})", 
                    i + 1, result.symbol, result.description, result.security_type);
            }
        }
        Err(e) => println!("Symbol search not available: {}", e),
    }

    // Sector metrics
    match client.misc().sector_metrics("NA").await {
        Ok(metrics) => {
            println!("\n📊 North American Sector Metrics:");
            for (i, sector) in metrics.data.iter().take(3).enumerate() {
                println!("  {}. {}", i + 1, sector.sector);
                println!("     Metrics available: {} indicators", sector.metrics.len());
            }
        }
        Err(e) => println!("Sector metrics not available: {}", e),
    }

    Ok(())
}