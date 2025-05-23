# finnhub-rs

A comprehensive Rust client for the [Finnhub.io](https://finnhub.io) financial data API.

## Features

- 🚀 Full async/await support with Tokio
- 📊 Complete API coverage (100+ endpoints)
- 🔒 Type-safe request and response models
- ⚡ Built-in rate limiting (30 requests/second)
- 🔄 WebSocket support for real-time data
- 🛡️ Comprehensive error handling
- 📝 Extensive documentation and examples

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
    let client = FinnhubClient::new("your-api-key");
    
    // Get a stock quote
    let quote = client.stock().quote("AAPL").await?;
    println!("AAPL price: ${}", quote.current_price);
    
    Ok(())
}
```

## Examples

See the [examples](examples/) directory for more detailed usage examples.

## API Coverage

- ✅ Stock fundamentals & market data
- ✅ Forex data
- ✅ Cryptocurrency data
- 🚧 Economic indicators
- 🚧 Alternative data
- 🚧 WebSocket streaming

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.