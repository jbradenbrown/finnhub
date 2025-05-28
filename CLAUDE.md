# Finnhub Rust API Client Architecture

## Project Overview

This is a comprehensive Rust client library for the Finnhub.io financial data API. The library is built using modern Rust patterns with async/await support via Tokio and HTTP requests via reqwest.

## Key Features

- **Full API Coverage**: Implements all 107 endpoints from the Finnhub API
- **Type Safety**: Strongly typed request/response models derived from OpenAPI spec
- **Async/Await**: Built on Tokio for high-performance async operations
- **Rate Limiting**: Built-in rate limiter respecting 30 requests/second limit
- **Error Handling**: Comprehensive error types with detailed context
- **WebSocket Support**: Real-time data streaming capabilities
- **Modular Design**: Organized by API categories for maintainability

## Architecture Design

### Core Components

```
finnhub-rs/
├── src/
│   ├── lib.rs              # Library entry point, re-exports
│   ├── client.rs           # Main FinnhubClient implementation
│   ├── auth.rs             # Authentication handling
│   ├── error.rs            # Error types and handling
│   ├── rate_limiter.rs     # Rate limiting implementation
│   ├── models/             # Shared data models
│   │   ├── mod.rs
│   │   └── common.rs       # Common types (timestamps, etc.)
│   ├── endpoints/          # API endpoint implementations
│   │   ├── mod.rs
│   │   ├── stock.rs        # Stock-related endpoints (53)
│   │   ├── forex.rs        # Forex endpoints (4)
│   │   ├── crypto.rs       # Crypto endpoints (4)
│   │   ├── bond.rs         # Bond endpoints (4)
│   │   ├── etf.rs          # ETF endpoints (4)
│   │   ├── mutual_fund.rs  # Mutual fund endpoints (6)
│   │   ├── economic.rs     # Economic data endpoints
│   │   ├── company.rs      # Company info endpoints
│   │   ├── news.rs         # News-related endpoints
│   │   ├── calendar.rs     # Calendar endpoints
│   │   └── misc.rs         # Other endpoints
│   └── websocket/          # WebSocket implementation
│       ├── mod.rs
│       └── stream.rs
├── examples/               # Usage examples
├── tests/                  # Integration tests
└── benches/               # Performance benchmarks
```

### Design Principles

1. **Builder Pattern**: Use builder pattern for complex requests
2. **Result Types**: All API calls return `Result<T, FinnhubError>`
3. **Zero-Copy Deserialization**: Use serde with borrowed data where possible
4. **Connection Pooling**: Reuse HTTP connections via reqwest
5. **Configurable Client**: Allow customization of timeouts, retries, etc.

### Client Architecture

```rust
pub struct FinnhubClient {
    api_key: String,
    http_client: reqwest::Client,
    rate_limiter: RateLimiter,
    base_url: String,
}
```

### Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum FinnhubError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Invalid API key")]
    Unauthorized,
    
    #[error("API error: {message}")]
    ApiError { message: String, code: u16 },
    
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
}
```

### Rate Limiting Design

- Token bucket algorithm with 30 tokens/second
- Automatic retry with exponential backoff
- Configurable burst capacity
- Per-client rate limiting

### Authentication Methods

1. **URL Parameter**: `?token=API_KEY`
2. **Header**: `X-Finnhub-Token: API_KEY`

The library will use header-based auth by default for security.

## API Categories Implementation Plan

### Priority 1 - Core Stock APIs (53 endpoints)
- Quote data
- Candles/OHLC
- Company profiles
- Financials
- Insider transactions
- Recommendations
- Price targets
- Technical indicators

### Priority 2 - Market Data
- Forex (4 endpoints)
- Crypto (4 endpoints)
- Indices (2 endpoints)

### Priority 3 - Alternative Data
- News & sentiment
- Economic indicators
- SEC filings
- ETF data
- Bond data

### Priority 4 - Advanced Features
- WebSocket streaming
- Batch requests
- Response caching

## Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }
url = "2"
futures = "0.3"
tokio-tungstenite = "0.20"  # For WebSocket
tracing = "0.1"
tower = { version = "0.4", features = ["limit", "retry"] }

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.5"
criterion = "0.5"
```

## Testing Strategy

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test against mock Finnhub API
3. **End-to-End Tests**: Optional tests against real API (requires API key)
4. **Property-based Tests**: Use proptest for edge cases

## Performance Considerations

1. **Connection Pooling**: Reuse HTTPS connections
2. **Response Streaming**: Stream large responses
3. **Parallel Requests**: Support concurrent API calls
4. **Caching**: Optional response caching layer
5. **Zero-Copy**: Minimize allocations in hot paths

## Security Considerations

1. **API Key Storage**: Never log or expose API keys
2. **HTTPS Only**: Enforce TLS for all connections
3. **Input Validation**: Validate all user inputs
4. **Error Messages**: Don't leak sensitive info in errors

## Development Workflow

1. **Branching Strategy**:
   - `main`: Stable releases
   - `develop`: Integration branch
   - `feature/*`: Feature branches
   - `fix/*`: Bug fix branches

2. **Commit Convention**:
   - `feat:` New features
   - `fix:` Bug fixes
   - `docs:` Documentation
   - `test:` Tests
   - `refactor:` Code refactoring
   - `perf:` Performance improvements

3. **Quality Checks**:
   - Run `cargo fmt` before commits
   - Run `cargo clippy` for lints
   - Run `cargo test` for all tests
   - Check documentation with `cargo doc`

## Future Enhancements

1. **Async Trait Support**: When stable
2. **Compile-time API Validation**: Macro-based endpoint generation
3. **WASM Support**: For browser usage
4. **CLI Tool**: Command-line interface for the API
5. **Response Middleware**: Pluggable response processing
6. **Metrics & Telemetry**: OpenTelemetry integration

## Release Plan

1. **v0.1.0**: Core client with stock endpoints
2. **v0.2.0**: Add forex, crypto, indices
3. **v0.3.0**: Add alternative data endpoints
4. **v0.4.0**: WebSocket support
5. **v1.0.0**: Stable API with all features

## API Implementation Status

- [x] Core Client Infrastructure
- [x] Authentication (header and URL parameter methods)
- [x] Rate Limiting (token bucket, 30 req/s)
- [x] Error Handling (comprehensive error types)
- [x] Basic Module Structure
- [ ] Stock Endpoints (17/54) - quote, company_profile, candles, financials, price_target, recommendations, insider_transactions, metrics, earnings, dividends, splits, symbols, historical_market_cap, historical_employee_count, historical_esg
- [ ] Forex Endpoints (1/4) - symbols
- [ ] Crypto Endpoints (2/4) - exchanges, symbols
- [ ] Bond Endpoints (0/4)
- [ ] ETF Endpoints (0/4)
- [ ] Mutual Fund Endpoints (0/6)
- [ ] Economic Data Endpoints (0/2)
- [ ] News Endpoints (0/3)
- [ ] Calendar Endpoints (0/3)
- [ ] Other Endpoints
- [x] WebSocket Support Structure (feature-gated)
- [x] Basic Example
- [x] README
- [ ] Comprehensive Tests
- [ ] Full API Documentation

## Progress Log

### 2025-05-23: Initial Setup
- Created comprehensive architecture documentation
- Set up Git repository with main/develop branches
- Implemented core library structure:
  - Client with configurable settings
  - Authentication module supporting both header and URL parameter methods
  - Token bucket rate limiter respecting 30 req/s limit
  - Comprehensive error types with thiserror
  - Basic endpoint structure for stock, forex, and crypto
  - WebSocket support structure (feature-gated)
- Added basic usage example
- Project successfully compiles

### Next Steps
1. Implement remaining stock endpoints using swagger.json as reference
2. Add comprehensive request/response models
3. Implement retry logic with exponential backoff
4. Add integration tests with mock server
5. Implement remaining endpoint categories

### 2025-05-28: API Specifications Split & Stock Candles
- Split swagger.json into 15 category-specific JSON files in api-specs/ directory
- Implemented stock candles (OHLCV) endpoint with CandleResolution enum
- Updated basic example to demonstrate candle data retrieval
- Organized 107 endpoints across categories: Stock (54), Forex (4), Crypto (4), etc.