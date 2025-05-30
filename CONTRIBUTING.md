# Contributing to finnhub-rs

Thank you for your interest in contributing to finnhub-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment (OS, Rust version, finnhub-rs version)
- Any relevant code snippets or error messages

### Suggesting Enhancements

Enhancement suggestions are welcome! Please provide:

- A clear and descriptive title
- Detailed description of the proposed feature
- Use cases and benefits
- Any relevant examples from other libraries

### Pull Requests

1. Fork the repository and create your branch from `develop`
2. If you've added code, add tests that cover your changes
3. Ensure all tests pass: `cargo test`
4. Make sure your code follows the style guidelines: `cargo fmt`
5. Ensure there are no clippy warnings: `cargo clippy -- -D warnings`
6. Update documentation for any API changes
7. Add an entry to CHANGELOG.md if appropriate

## Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/finnhub-rs
cd finnhub-rs

# Add upstream remote
git remote add upstream https://github.com/jbradenbrown/finnhub-rs

# Create a feature branch
git checkout -b feature/your-feature-name

# Install development dependencies
cargo build

# Run tests (requires API key)
FINNHUB_API_KEY=your_key cargo test

# Run a specific test
FINNHUB_API_KEY=your_key cargo test test_name

# Run examples
FINNHUB_API_KEY=your_key cargo run --example basic_usage
```

## Style Guidelines

### Rust Code Style

- Follow standard Rust naming conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Prefer explicit types for public APIs
- Add documentation comments for all public items
- Use `#[must_use]` where appropriate

### Commit Messages

Follow the conventional commits specification:

```
type(scope): subject

body

footer
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat(stock): add insider sentiment endpoint

- Add insider_sentiment() method to StockEndpoints
- Add InsiderSentimentData model
- Update documentation and examples
```

### Documentation

- All public APIs must have documentation comments
- Include examples in doc comments where helpful
- Update README.md for significant features
- Keep CLAUDE.md updated with architectural decisions

### Testing

- Write unit tests for model serialization/deserialization
- Write integration tests for new endpoints (using mocks when possible)
- Test error conditions, not just happy paths
- Use descriptive test names that explain what is being tested

Example test:
```rust
#[tokio::test]
async fn test_stock_quote_handles_invalid_symbol() {
    let client = FinnhubClient::new("test_key");
    let result = client.stock().quote("INVALID123").await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotFound => {}
        e => panic!("Expected NotFound error, got: {:?}", e),
    }
}
```

## Project Structure

```
finnhub-rs/
├── src/
│   ├── client.rs           # Main client implementation
│   ├── endpoints/          # API endpoint implementations
│   │   ├── stock.rs        # Stock market endpoints
│   │   └── ...             # Other endpoint categories
│   ├── models/             # Response/request models
│   │   ├── stock/          # Stock-related models
│   │   └── ...             # Other model categories
│   ├── error.rs            # Error types
│   ├── auth.rs             # Authentication
│   └── rate_limiter.rs     # Rate limiting
├── examples/               # Usage examples
├── tests/                  # Integration tests
└── benches/               # Performance benchmarks
```

## Adding New Endpoints

1. Check the [Finnhub API documentation](https://finnhub.io/docs/api)
2. Add the endpoint method to the appropriate file in `src/endpoints/`
3. Create request/response models in `src/models/`
4. Add tests for the new endpoint
5. Update the API coverage in README.md and CLAUDE.md
6. Add an example if the endpoint introduces new concepts

Example:
```rust
// In src/endpoints/stock.rs
impl<'a> StockEndpoints<'a> {
    /// Get company ESG scores.
    pub async fn esg(&self, symbol: &str) -> Result<ESGScore> {
        self.client
            .get(&format!("/stock/esg?symbol={}", symbol))
            .await
    }
}

// In src/models/stock/compliance.rs
/// ESG (Environmental, Social, Governance) scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ESGScore {
    /// Symbol
    pub symbol: String,
    /// ESG scores
    pub esg_scores: Vec<ESGData>,
}
```

## Development Tools

This project was initially developed using [Claude Code](https://github.com/anthropics/claude-code), and we welcome contributions made with AI assistance. If you use AI tools in your contributions:

1. Ensure you understand and can explain all code you submit
2. Test thoroughly - AI-generated code may have subtle issues
3. Follow all the same quality standards as human-written code
4. Consider mentioning AI assistance in your PR for transparency

## Questions?

Feel free to open an issue for any questions about contributing. We're here to help!