# finnhub

A comprehensive Rust client for the [Finnhub.io](https://finnhub.io) financial data API.

[![Crates.io](https://img.shields.io/crates/v/finnhub.svg)](https://crates.io/crates/finnhub)
[![Documentation](https://docs.rs/finnhub/badge.svg)](https://docs.rs/finnhub)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Features

- 🚀 Full async/await support with Tokio
- 📊 Near-complete API coverage (122/123 endpoints - 99.2%)
- 🔒 Type-safe request and response models
- ⚡ Built-in rate limiting (30 requests/second)
- 🔄 WebSocket support (minimal implementation, feature-gated)
- 🛡️ Comprehensive error handling
- 📝 Well-organized module structure
- 🎯 Zero-copy deserialization where possible
- 📨 GET and POST request support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
finnhub = "0.3"

# For WebSocket support
finnhub = { version = "0.3", features = ["websocket"] }
```

## Quick Start

```rust
use finnhub::{FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with your API key
    let client = FinnhubClient::new("your-api-key");
    
    // Get a stock quote
    let quote = client.stock().quote("AAPL").await?;
    println!("AAPL price: ${:.2}", quote.current_price);
    
    // Get company profile
    let profile = client.stock().company_profile("AAPL").await?;
    println!("Company: {}", profile.name.unwrap_or_default());
    
    // Search for symbols
    let results = client.misc().symbol_search("apple", Some("US")).await?;
    println!("Found {} results for 'apple'", results.count);
    
    Ok(())
}
```

## Authentication

The library uses header authentication (`X-Finnhub-Token`) by default for better security. Both header and URL parameter authentication are supported by Finnhub.

```rust
use finnhub::{FinnhubClient, ClientConfig, auth::AuthMethod};

// Default: Header authentication (more secure)
let client = FinnhubClient::new("your-api-key");

// Alternative: URL parameter authentication
let config = ClientConfig {
    auth_method: AuthMethod::UrlParameter,
    ..ClientConfig::default()
};
let client = FinnhubClient::with_config("your-api-key", config);
```

## API Coverage

### Stock Market Data (66/66 endpoints)
- ✅ **Quotes & Prices**: Real-time quotes, candles (OHLCV), ⚠️ bid/ask *[Premium]*, ⚠️ tick data *[Premium]*
- ✅ **Options** *(new in 0.3)*: Full option chain with Greeks, OI, IV, theoretical/intrinsic/time value
- ✅ **Company Info**: Profile (basic + ⚠️ premium), peers, executives, ⚠️ market cap history *[Premium]*
- ✅ **Fundamentals**: Financials, metrics, earnings, dividends, ⚠️ revenue breakdown v1+v2 *[v2 Premium]*
- ✅ **Estimates**: Price targets, recommendations, EPS/revenue/EBIT/EBITDA/net-income/pretax/gross-income/DPS estimates, earnings quality
- ⚠️ **Alternative Data**: ESG scores, patents, visa applications, lobbying *[Most require Premium]*
- ✅ **Insider & Ownership**: Insider transactions, ⚠️ sentiment *[Premium]*, fund ownership, ⚠️ institutional 13-F profile/portfolio/ownership *[Premium]*
- ✅ **Corporate Actions**: Dividends, splits, ⚠️ symbol/ISIN changes *[Premium]*
- ✅ **Market Info**: Symbols, market status, holidays
- ✅ **Filings**: SEC filings, transcripts, presentations, similarity index

### Other Markets
- ✅ **ETFs** (5/5): Profile, holdings, country/sector exposure, allocation *(new in 0.3)*
- ⚠️ **Forex** (4/4): Symbols, candles, rates, exchanges *[Premium]*
- ⚠️ **Crypto** (4/4): Exchanges, symbols, candles, profile *[Premium]*
- ⚠️ **Bonds** (4/4): Profile, price, tick data, yield curve *[Premium]*
- ⚠️ **Mutual Funds** (6/6): Profile, holdings, performance, ESG data *[Premium]*
- ⚠️ **Indices** (2/2): Constituents, historical constituents *[Premium]*

### Data & Analytics
- ✅ **Economic Data** (2/2): Economic indicators and codes
- ✅ **News** (4/4): Market news, company news, sentiment, newsroom *(new in 0.3)*
- ✅ **Calendar** (3/3): Earnings, economic events, IPO calendar
- ✅ **Technical Analysis** (3/3): Pattern recognition, support/resistance, aggregate indicators
- ⚠️ **Global Filings** *(new in 0.3, 3/4)*: Filter, search, in-filing search *[Premium]* — `download` deferred (raw bytes)

### Miscellaneous
- ✅ **Search & Lookup**: Symbol search, country metadata
- ✅ **Alternative Data**: COVID-19, FDA calendar, airline price index, bank branches *(new in 0.3)*
- ✅ **Market Analysis**: Sector metrics, press releases, technical indicators
- ✅ **AI Features**: AI chat *(now functional in 0.3 via POST support)*

### Advanced Features
- ⚠️ **WebSocket**: Basic structure only (not production-ready)
- ✅ **Rate Limiting**: Automatic 30 req/s limit with flexible strategies
- ✅ **Error Handling**: Typed errors with context and retry helpers

## Examples

### Stock Market Data
```rust
use finnhub::models::stock::{StatementType, StatementFrequency};

// Get financials
let financials = client.stock()
    .financials("AAPL", StatementType::IncomeStatement, StatementFrequency::Annual)
    .await?;

// Get insider transactions
let insiders = client.stock().insider_transactions("AAPL").await?;

// Get price target consensus
let target = client.stock().price_target("AAPL").await?;
println!("Average target: ${:.2}", target.target_mean);
```

### Option Chains

```rust
let chain = client.stock().option_chain("AAPL").await?;
println!("Underlying last price: ${:.2}", chain.last_trade_price.unwrap_or_default());

if let Some(expiry) = chain.data.first() {
    println!(
        "Expiration {}: {} calls / {} puts, IV {:.2}%",
        expiry.expiration_date,
        expiry.options.call.len(),
        expiry.options.put.len(),
        expiry.implied_volatility.unwrap_or_default(),
    );

    if let Some(c) = expiry.options.call.first() {
        println!(
            "  {} strike ${} bid {:?} ask {:?} delta {:?}",
            c.contract_name, c.strike, c.bid, c.ask, c.delta,
        );
    }
}
```

### Institutional 13-F Holdings

```rust
// Berkshire Hathaway portfolio (CIK 1067983)
let portfolio = client.stock()
    .institutional_portfolio("1067983", "2024-01-01", "2024-06-30")
    .await?;

for snapshot in &portfolio.data {
    println!("Filed {}: {} positions", snapshot.filing_date, snapshot.portfolio.len());
}

// Who holds AAPL?
let holders = client.stock()
    .institutional_ownership("AAPL", "", "2024-01-01", "2024-06-30")
    .await?;
```

### Alternative Data

Many alternative data endpoints require premium API access:

```rust
// Social sentiment (available with basic access)
let sentiment = client.stock()
    .social_sentiment("AAPL", "2024-01-01", "2024-01-07")
    .await?;
println!("Symbol: {}", sentiment.symbol);
println!("Total data points: {}", sentiment.data.len());

// Premium endpoints (require additional access):
// - ESG scores: client.stock().esg("AAPL")
// - Patent applications: client.stock().uspto_patents("NVDA", from, to)  
// - Congressional trading: client.stock().congressional_trading("AAPL", None, None)
// - Lobbying data: client.stock().lobbying("AAPL", from, to)
```

### Calendar Events
```rust
// Earnings calendar
let earnings = client.calendar()
    .earnings(Some("2024-01-01"), Some("2024-01-07"), None)
    .await?;
println!("Upcoming earnings: {} companies", earnings.earnings_calendar.len());

// IPO calendar  
let ipos = client.calendar()
    .ipo("2024-01-01", "2024-01-31")
    .await?;
println!("Recent IPOs: {} companies", ipos.ipo_calendar.len());
```

### News & Sentiment
```rust
use finnhub::models::news::NewsCategory;

// Company news with sentiment
let news = client.news().company_news("AAPL", "2024-12-01", "2024-12-07").await?;

// Market-wide news
let market_news = client.news().market_news(NewsCategory::General, None).await?;
```

### Technical Analysis
```rust
// Support and resistance levels
let levels = client.scanner().support_resistance("AAPL", "D").await?;

// Aggregate technical indicators
let indicators = client.scanner().aggregate_indicators("AAPL", "D").await?;
println!("Signal: {} (Buy: {}, Sell: {})", 
    indicators.technical_analysis.signal,
    indicators.technical_analysis.count.buy,
    indicators.technical_analysis.count.sell
);
```

### Search & Discovery
```rust
// Symbol search
let results = client.misc().symbol_search("tesla", Some("US")).await?;
println!("Found {} results", results.count);

// Country information
let countries = client.misc().country().await?;
println!("Available in {} countries", countries.len());

// FDA calendar
let fda = client.misc().fda_calendar().await?;
println!("Upcoming FDA events: {}", fda.len());
```

## Project Structure

```
finnhub/
├── src/
│   ├── client.rs           # Main client implementation (GET + POST)
│   ├── auth.rs             # Authentication handling
│   ├── error.rs            # Error types
│   ├── rate_limiter.rs     # Rate limiting
│   ├── models/             # Response models
│   │   ├── stock/          # Stock models (15 modules, incl. options)
│   │   ├── global_filings.rs # Global filings search models
│   │   ├── forex.rs        # Forex models
│   │   ├── crypto.rs       # Crypto models
│   │   └── ...             # Other market models
│   └── endpoints/          # API endpoint implementations
│       ├── stock/          # Stock endpoints (15 modules)
│       ├── global_filings.rs # /global-filings endpoints
│       ├── forex.rs        # Forex endpoints
│       └── ...             # Other endpoints
└── examples/               # Usage examples
```

## Error Handling

The library provides comprehensive error handling:

```rust
use finnhub::Error;

match client.stock().quote("AAPL").await {
    Ok(quote) => println!("Price: ${}", quote.current_price),
    Err(Error::RateLimitExceeded { retry_after }) => {
        println!("Rate limit hit, retry after {} seconds", retry_after);
    },
    Err(Error::Unauthorized) => {
        println!("Invalid API key");
    },
    Err(e) => println!("Error: {}", e),
}
```

## Rate Limiting

The client includes built-in rate limiting to comply with Finnhub's API limits:

```rust
// Default: 30 requests/second with burst capacity
let client = FinnhubClient::new("your-api-key");

// For batch processing: 15-second window (450 request burst)
let mut config = ClientConfig::default();
config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
let client = FinnhubClient::with_config("your-api-key", config);

// Rate limiting is automatic
for symbol in ["AAPL", "GOOGL", "MSFT"] {
    let quote = client.stock().quote(symbol).await?;
    // Client automatically manages request rate
}
```

## Production Best Practices

### Retry Logic

This library intentionally does not implement automatic retry logic, allowing applications to implement context-aware retry strategies. The library provides helpers to make this easy:

```rust
use finnhub::{Error, Result};
use std::time::Duration;
use tokio::time::sleep;

async fn with_retry<T, F, Fut>(mut f: F, max_attempts: u32) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if e.is_retryable() && attempt < max_attempts => {
                let delay = e.retry_after()
                    .unwrap_or(1)  // Default 1 second
                    .max(1);       // At least 1 second
                sleep(Duration::from_secs(delay)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}

// Usage
let quote = with_retry(|| client.stock().quote("AAPL"), 3).await?;
```

### Caching

Response caching is best implemented at the application layer where you understand data freshness requirements:

```rust
use finnhub::models::stock::Quote;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CachedQuote {
    quote: Quote,
    fetched_at: Instant,
}

struct QuoteCache {
    cache: HashMap<String, CachedQuote>,
    ttl: Duration,
}

impl QuoteCache {
    async fn get_quote(&mut self, client: &FinnhubClient, symbol: &str) -> Result<Quote> {
        if let Some(cached) = self.cache.get(symbol) {
            if cached.fetched_at.elapsed() < self.ttl {
                return Ok(cached.quote.clone());
            }
        }
        
        let quote = client.stock().quote(symbol).await?;
        self.cache.insert(symbol.to_string(), CachedQuote {
            quote: quote.clone(),
            fetched_at: Instant::now(),
        });
        Ok(quote)
    }
}
```

### Error Handling

Always handle specific error types appropriately:

```rust
match client.stock().quote("AAPL").await {
    Ok(quote) => process_quote(quote),
    Err(Error::RateLimitExceeded { retry_after }) => {
        // Back off and retry later
        sleep(Duration::from_secs(retry_after)).await;
    }
    Err(Error::Unauthorized) => {
        // Check API key configuration
        panic!("Invalid API key");
    }
    Err(e) => {
        // Log and handle other errors
        eprintln!("API error: {}", e);
    }
}
```

### Concurrent Requests

When making multiple requests, consider rate limits and use concurrency control:

```rust
use futures::stream::{self, StreamExt};

let symbols = vec!["AAPL", "GOOGL", "MSFT", "AMZN", "FB"];

// Process 3 symbols concurrently to stay well under rate limit
let quotes: Vec<_> = stream::iter(symbols)
    .map(|symbol| async move {
        client.stock().quote(symbol).await
    })
    .buffer_unordered(3)
    .collect()
    .await;
```

## WebSocket Support (Minimal)

Basic WebSocket structure is implemented but requires significant work:

```rust
// Requires 'websocket' feature
use finnhub::websocket::{WebSocketClient, WebSocketMessage};

let client = WebSocketClient::new("your-api-key");
let mut stream = client.connect().await?;

// Subscribe to symbols
stream.subscribe("AAPL").await?;

// Process messages
match stream.next().await? {
    Some(WebSocketMessage::Trade { data }) => {
        for trade in data {
            println!("Trade: {} @ ${}", trade.symbol, trade.price);
        }
    }
    Some(WebSocketMessage::Ping) => {
        println!("Received ping");
    }
    Some(WebSocketMessage::Error { msg }) => {
        eprintln!("Error: {}", msg);
    }
    None => println!("Stream closed"),
}
```

See `examples/websocket_basic.rs` for a complete example.

**Note**: WebSocket support is minimal and not recommended for production use. It lacks:
- Automatic reconnection
- Heartbeat handling  
- Convenient subscription methods
- Proper error recovery

## Environment Variables

For examples and tests, you can use environment variables:

```bash
# .env file
FINNHUB_API_KEY=your_api_key_here
```

```rust
// In your code
dotenv::dotenv().ok();
let api_key = std::env::var("FINNHUB_API_KEY")
    .expect("FINNHUB_API_KEY must be set");
```

## Troubleshooting

### Common Issues

1. **401 Unauthorized**: Check your API key is valid and has appropriate permissions
2. **429 Rate Limit**: You're exceeding 30 requests/second. The client should handle this automatically
3. **Empty responses**: Some endpoints return empty data outside market hours or for invalid symbols

### Debug Logging

Enable debug logging to see request details:

```bash
RUST_LOG=finnhub=debug cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CLAUDE.md](CLAUDE.md) for development guidelines and architecture details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/jbradenbrown/finnhub
cd finnhub

# Run tests (requires API key)
FINNHUB_API_KEY=your_key cargo test

# Run specific example
FINNHUB_API_KEY=your_key cargo run --example basic_usage

# Check formatting and lints
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Development

This library was developed with assistance from [Claude](https://claude.ai), Anthropic's AI assistant, using the [Claude Code](https://github.com/anthropics/claude-code) development environment. The AI helped with implementation, documentation, and best practices while maintaining human oversight and decision-making throughout the development process.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.