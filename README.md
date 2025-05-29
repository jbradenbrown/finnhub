# finnhub-rs

A comprehensive Rust client for the [Finnhub.io](https://finnhub.io) financial data API.

[![Crates.io](https://img.shields.io/crates/v/finnhub.svg)](https://crates.io/crates/finnhub)
[![Documentation](https://docs.rs/finnhub/badge.svg)](https://docs.rs/finnhub)
[![License](https://img.shields.io/crates/l/finnhub.svg)](LICENSE-MIT)

## Features

- 🚀 Full async/await support with Tokio
- 📊 Extensive API coverage (103/107 endpoints - 96.3%)
- 🔒 Type-safe request and response models
- ⚡ Built-in rate limiting (30 requests/second)
- 🔄 WebSocket support for real-time data (feature-gated)
- 🛡️ Comprehensive error handling
- 📝 Well-organized module structure
- 🎯 Zero-copy deserialization where possible

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
finnhub = "0.1.0"

# For WebSocket support
finnhub = { version = "0.1.0", features = ["websocket"] }
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
    
    // Get forex rates
    let rates = client.forex().rates("USD").await?;
    println!("USD/EUR: {}", rates.quote.get("EUR").unwrap_or(&0.0));
    
    Ok(())
}
```

## Authentication

The library uses URL parameter authentication (`?token=YOUR_API_KEY`) by default, which is Finnhub's recommended method.

```rust
use finnhub::{FinnhubClient, ClientConfig, auth::AuthMethod};

// Default: URL parameter authentication (recommended)
let client = FinnhubClient::new("your-api-key");

// Alternative: Header authentication
let config = ClientConfig {
    auth_method: AuthMethod::Header,
    ..ClientConfig::default()
};
let client = FinnhubClient::with_config("your-api-key", config);
```

## API Coverage

### Stock Market Data (52/54 endpoints - 96.3%)
- ✅ **Quotes & Prices**: Real-time quotes, bid/ask, candles (OHLCV)
- ✅ **Company Info**: Profile, peers, executives, market cap history
- ✅ **Fundamentals**: Financials, metrics, earnings, dividends
- ✅ **Estimates**: Price targets, recommendations, earnings estimates
- ✅ **Alternative Data**: ESG scores, patents, visa applications, lobbying
- ✅ **Insider Data**: Transactions, sentiment, ownership
- ✅ **Market Info**: Symbols, market status, holidays

### Other Markets
- ✅ **Forex** (4/4): Symbols, candles, rates, exchanges
- ✅ **Crypto** (4/4): Exchanges, symbols, candles, profile
- ✅ **Bonds** (4/4): Profile, price, tick data, yield curve
- ✅ **ETFs** (4/4): Profile, holdings, country/sector exposure
- ✅ **Mutual Funds** (6/6): Profile, holdings, performance, ESG data
- ✅ **Indices** (2/2): Constituents, historical constituents

### Data & Analytics
- ✅ **Economic Data** (2/2): Economic indicators and codes
- ✅ **News** (3/3): Market news, company news, sentiment
- ✅ **Calendar** (3/3): Earnings, economic events, IPO calendar
- ✅ **Technical Analysis** (3/3): Pattern recognition, support/resistance, aggregate indicators

### Miscellaneous
- ✅ **Search & Lookup**: Symbol search, country metadata
- ✅ **Alternative Data**: COVID-19, FDA calendar, airline price index
- ✅ **Market Analysis**: Sector metrics, press releases, technical indicators
- 🚧 **AI Features**: AI chat (requires POST support)

### Advanced Features
- 🚧 **WebSocket**: Real-time streaming (structure implemented)
- ✅ **Rate Limiting**: Automatic 30 req/s limit
- ✅ **Error Handling**: Typed errors with context

## Examples

### Stock Market Data
```rust
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

### Alternative Data
```rust
// ESG scores
let esg = client.stock().esg("AAPL").await?;

// Patent applications
let patents = client.stock().uspto_patents("NVDA", "2023-01-01", "2023-12-31").await?;

// Congressional trading
let trades = client.stock().congressional_trading("AAPL", None, None).await?;
```

### Market Indices
```rust
// S&P 500 constituents
let sp500 = client.index().constituents("^GSPC").await?;
println!("S&P 500 has {} companies", sp500.constituents.len());
```

### News & Sentiment
```rust
// Company news with sentiment
let news = client.news().company_news("AAPL", "2024-01-01", "2024-01-31").await?;

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

// Country information
let countries = client.misc().country().await?;

// Sector metrics
let sectors = client.misc().sector_metrics("NA").await?;
```

## Project Structure

```
finnhub-rs/
├── src/
│   ├── client.rs           # Main client implementation
│   ├── auth.rs             # Authentication handling
│   ├── error.rs            # Error types
│   ├── rate_limiter.rs     # Rate limiting
│   ├── models/             # Response models
│   │   ├── stock/          # Stock models (organized by category)
│   │   ├── forex.rs        # Forex models
│   │   ├── crypto.rs       # Crypto models
│   │   └── ...             # Other market models
│   └── endpoints/          # API endpoint implementations
│       ├── stock.rs        # Stock endpoints
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

The client includes built-in rate limiting (30 requests/second) to comply with Finnhub's API limits:

```rust
// Rate limiting is automatic
for symbol in ["AAPL", "GOOGL", "MSFT"] {
    let quote = client.stock().quote(symbol).await?;
    // Client automatically manages request rate
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CLAUDE.md](CLAUDE.md) for development guidelines and architecture details.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.