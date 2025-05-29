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
    println!("  Name: {}", profile.name.unwrap_or_else(|| "N/A".to_string()));
    println!("  Country: {}", profile.country.unwrap_or_else(|| "N/A".to_string()));
    println!("  Exchange: {}", profile.exchange.unwrap_or_else(|| "N/A".to_string()));
    println!("  Industry: {}", profile.finnhub_industry.unwrap_or_else(|| "N/A".to_string()));
    if let Some(market_cap) = profile.market_capitalization {
        println!("  Market Cap: ${:.2}B", market_cap / 1_000_000_000.0);
    } else {
        println!("  Market Cap: N/A");
    }

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
            println!("  Last Updated: {}", target.last_updated);
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
            for data_point in historical_data.data.iter().rev().take(5) {
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

    // Get IPO calendar
    println!("\nFetching IPO calendar...");
    let ipo_to = Utc::now().format("%Y-%m-%d").to_string();
    let ipo_from = (Utc::now() - Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();
    
    match client.calendar().ipo(&ipo_from, &ipo_to).await {
        Ok(calendar) => {
            println!("Recent and upcoming IPOs:");
            for ipo in calendar.ipo_calendar.iter().take(5) {
                if let (Some(symbol), Some(name), Some(date)) = (&ipo.symbol, &ipo.name, &ipo.date) {
                    println!("  📈 {} ({}) - {}", symbol, name, date);
                    if let Some(price) = &ipo.price {
                        println!("     Price range: {}", price);
                    }
                    if let Some(status) = &ipo.status {
                        println!("     Status: {}", status);
                    }
                }
            }
            if calendar.ipo_calendar.is_empty() {
                println!("  No IPOs found in the specified date range");
            }
        }
        Err(e) => println!("IPO calendar not available: {}", e),
    }

    // Get SEC filings
    println!("\nFetching AAPL SEC filings...");
    match client.stock().sec_filings(Some("AAPL"), None, None, None, None, None).await {
        Ok(filings) => {
            println!("Recent SEC filings:");
            for filing in filings.iter().take(5) {
                if let (Some(form), Some(filed_date)) = (&filing.form, &filing.filed_date) {
                    println!("  📄 Form {}: {}", form, filed_date);
                    if let Some(url) = &filing.filing_url {
                        println!("     URL: {}", url);
                    }
                }
            }
        }
        Err(e) => println!("SEC filings not available: {}", e),
    }

    // Get bid-ask data
    println!("\nFetching AAPL bid-ask data...");
    match client.stock().bid_ask("AAPL").await {
        Ok(bid_ask) => {
            if let (Some(bid), Some(ask)) = (bid_ask.bid, bid_ask.ask) {
                println!("Bid-Ask Spread:");
                println!("  Bid: ${:.2}", bid);
                println!("  Ask: ${:.2}", ask);
                println!("  Spread: ${:.2}", ask - bid);
                if let (Some(bv), Some(av)) = (bid_ask.bid_volume, bid_ask.ask_volume) {
                    println!("  Bid Volume: {:.0}", bv);
                    println!("  Ask Volume: {:.0}", av);
                }
            }
        }
        Err(e) => println!("Bid-ask data not available: {}", e),
    }

    // Get tick data (limited example)
    println!("\nFetching AAPL tick data...");
    let tick_date = (Utc::now() - Duration::days(1)).format("%Y-%m-%d").to_string();
    match client.stock().tick_data("AAPL", &tick_date, 100, 0).await {
        Ok(ticks) => {
            println!("Tick data for {} (showing first 5 of {}):", tick_date, ticks.count);
            for i in 0..5.min(ticks.price.len()) {
                println!(
                    "  Time: {} | Price: ${:.2} | Volume: {:.0}",
                    chrono::DateTime::from_timestamp_millis(ticks.timestamp[i])
                        .map(|dt| dt.format("%H:%M:%S").to_string())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    ticks.price[i],
                    ticks.volume[i]
                );
            }
            println!("  Total ticks available: {}", ticks.total);
        }
        Err(e) => println!("Tick data not available: {}", e),
    }

    // Get financials as reported
    println!("\nFetching AAPL financials as reported...");
    match client.stock().financials_reported(Some("AAPL"), None, None, Some("quarterly")).await {
        Ok(financials) => {
            println!("Financials as reported (quarterly):");
            for report in financials.data.iter().take(2) {
                if let (Some(year), Some(quarter), Some(form)) = (report.year, report.quarter, &report.form) {
                    println!("  📊 {} - Q{} {} (filed: {})", 
                        year, quarter, form, 
                        report.filed_date.as_deref().unwrap_or("Unknown")
                    );
                }
            }
            println!("  Total reports available: {}", financials.data.len());
        }
        Err(e) => println!("Financials as reported not available: {}", e),
    }

    // Get earnings calendar
    println!("\nFetching earnings calendar...");
    let earnings_to = Utc::now().format("%Y-%m-%d").to_string();
    let earnings_from = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    
    match client.calendar().earnings(Some(&earnings_from), Some(&earnings_to), None).await {
        Ok(calendar) => {
            println!("Recent earnings releases:");
            for earning in calendar.earnings_calendar.iter().take(5) {
                if let Some(symbol) = &earning.symbol {
                    println!("  📊 {} - {} ({})", 
                        symbol, 
                        earning.date.as_deref().unwrap_or("Unknown"),
                        earning.hour.as_deref().unwrap_or("Unknown")
                    );
                    if let (Some(est), Some(act)) = (earning.eps_estimate, earning.eps_actual) {
                        println!("     EPS: ${:.2} actual vs ${:.2} estimate", act, est);
                    }
                }
            }
        }
        Err(e) => println!("Earnings calendar not available: {}", e),
    }

    // Get economic calendar
    println!("\nFetching economic calendar...");
    match client.calendar().economic(None, None).await {
        Ok(calendar) => {
            println!("Upcoming economic events:");
            for event in calendar.economic_calendar.iter().take(5) {
                if let Some(event_name) = &event.event {
                    println!("  📈 {} - {} ({})", 
                        event_name,
                        event.country.as_deref().unwrap_or("Unknown"),
                        event.time.as_deref().unwrap_or("Unknown")
                    );
                    if let Some(impact) = &event.impact {
                        println!("     Impact: {}", impact);
                    }
                }
            }
        }
        Err(e) => println!("Economic calendar not available: {}", e),
    }

    // Get ETF profile
    println!("\nFetching SPY ETF profile...");
    match client.etf().profile(Some("SPY"), None).await {
        Ok(profile) => {
            println!("ETF Profile:");
            if let Some(name) = &profile.name {
                println!("  Name: {}", name);
            }
            if let Some(aum) = profile.aum {
                println!("  AUM: ${:.2}B", aum / 1_000_000_000.0);
            }
            if let Some(expense) = profile.expense_ratio {
                println!("  Expense Ratio: {:.2}%", expense);
            }
            if let Some(company) = &profile.etf_company {
                println!("  Issuer: {}", company);
            }
        }
        Err(e) => println!("ETF profile not available: {}", e),
    }

    // Get ETF holdings
    println!("\nFetching SPY ETF holdings...");
    match client.etf().holdings(Some("SPY"), None, None, None).await {
        Ok(holdings) => {
            println!("Top ETF Holdings:");
            for holding in holdings.holdings.iter().take(5) {
                if let (Some(symbol), Some(name), Some(percent)) = (&holding.symbol, &holding.name, holding.percent) {
                    println!("  {} ({}) - {:.2}%", symbol, name, percent);
                }
            }
        }
        Err(e) => println!("ETF holdings not available: {}", e),
    }

    // Get market holidays
    println!("\nFetching US market holidays...");
    match client.stock().market_holiday("US").await {
        Ok(holidays) => {
            println!("Market Holidays for {}:", holidays.exchange);
            for holiday in holidays.data.iter().take(5) {
                println!("  🎉 {} - {}", holiday.event_name, holiday.at_date);
                if !holiday.trading_hour.is_empty() {
                    println!("     Trading Hours: {}", holiday.trading_hour);
                }
            }
        }
        Err(e) => println!("Market holidays not available: {}", e),
    }

    // Get investment theme
    println!("\nFetching investment theme (financial exchanges)...");
    match client.stock().investment_theme("financialExchangesData").await {
        Ok(theme) => {
            println!("Investment Theme: {}", theme.theme);
            println!("Stocks in theme:");
            for stock in theme.data.iter().take(5) {
                println!("  - {}", stock.symbol);
            }
        }
        Err(e) => println!("Investment theme not available: {}", e),
    }

    // Get transcripts list
    println!("\nFetching AAPL earnings call transcripts list...");
    match client.stock().transcripts_list("AAPL").await {
        Ok(list) => {
            println!("Available Transcripts for {}:", list.symbol);
            for transcript in list.transcripts.iter().take(3) {
                println!("  📞 {} - Q{} {} ({})", 
                    transcript.title, 
                    transcript.quarter, 
                    transcript.year,
                    transcript.time
                );
                println!("     ID: {}", transcript.id);
            }
        }
        Err(e) => println!("Transcripts list not available: {}", e),
    }

    // Get ESG scores
    println!("\nFetching AAPL ESG scores...");
    match client.stock().esg("AAPL").await {
        Ok(esg) => {
            println!("ESG Scores for {}:", esg.company_name);
            if let Some(rating) = esg.esg_risk_rating {
                println!("  Overall ESG Risk Rating: {:.1}", rating);
            }
            if let Some(level) = &esg.esg_risk_level {
                println!("  Risk Level: {}", level);
            }
            if let Some(env) = esg.environment_risk_score {
                println!("  Environmental Risk: {:.1}", env);
            }
            if let Some(social) = esg.social_risk_score {
                println!("  Social Risk: {:.1}", social);
            }
            if let Some(gov) = esg.governance_risk_score {
                println!("  Governance Risk: {:.1}", gov);
            }
        }
        Err(e) => println!("ESG scores not available: {}", e),
    }

    // Get S&P 500 constituents
    println!("\nFetching S&P 500 constituents...");
    match client.index().constituents("^GSPC").await {
        Ok(constituents) => {
            println!("S&P 500 Index ({}):", constituents.symbol);
            println!("Total constituents: {}", constituents.constituents.len());
            println!("Top 5 by weight:");
            for (i, constituent) in constituents.constituents_breakdown.iter().take(5).enumerate() {
                if let Some(weight) = constituent.weight {
                    println!("  {}. {} ({}) - {:.2}%", i + 1, constituent.name, constituent.symbol, weight);
                }
            }
        }
        Err(e) => println!("Index constituents not available: {}", e),
    }

    // Symbol search
    println!("\nSearching for Apple symbols...");
    match client.misc().symbol_search("apple", Some("US")).await {
        Ok(results) => {
            println!("Found {} results for 'apple':", results.count);
            for result in results.result.iter().take(5) {
                println!("  {} ({}) - {}", result.symbol, result.display_symbol, result.description);
            }
        }
        Err(e) => println!("Symbol search not available: {}", e),
    }

    // Country metadata
    println!("\nFetching country metadata...");
    match client.misc().country().await {
        Ok(countries) => {
            println!("Available countries:");
            for country in countries.iter().take(3) {
                println!("  {} ({}) - Currency: {} ({})", 
                    country.country, 
                    country.code2, 
                    country.currency, 
                    country.currency_code
                );
                if let Some(rating) = &country.rating {
                    println!("    Credit Rating: {}", rating);
                }
            }
            println!("  ... and {} more countries", countries.len() - 3);
        }
        Err(e) => println!("Country metadata not available: {}", e),
    }

    // Technical analysis indicators
    println!("\nFetching aggregate technical indicators for AAPL...");
    match client.scanner().aggregate_indicators("AAPL", "D").await {
        Ok(indicators) => {
            println!("Technical Analysis for AAPL:");
            println!("  Signal: {}", indicators.technical_analysis.signal);
            println!("  Buy signals: {}", indicators.technical_analysis.count.buy);
            println!("  Neutral signals: {}", indicators.technical_analysis.count.neutral);
            println!("  Sell signals: {}", indicators.technical_analysis.count.sell);
            println!("  Trending: {} (ADX: {:.2})", indicators.trend.trending, indicators.trend.adx);
        }
        Err(e) => println!("Technical indicators not available: {}", e),
    }

    Ok(())
}
