# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Flexible rate limiting with 15-second window support
  - `RateLimitStrategy` enum with `PerSecond`, `FifteenSecondWindow`, and `Custom` options
  - Allows burst processing while maintaining 30 req/s average
- Comprehensive test suite for rate limiting behavior
- Additional examples: `rate_limiting_demo`, `rate_limiting_explanation`, `error_handling`
- Production best practices documentation

### Changed
- Changed default authentication method from URL parameter to header authentication
  - Header authentication (`X-Finnhub-Token`) is now the default for better security
  - URL parameter authentication (`?token=API_KEY`) is still supported via `AuthMethod::UrlParameter`
  - Both methods continue to work with all endpoints
  - Updated all examples and documentation to reflect this change
- Enhanced README with retry logic patterns, caching examples, and troubleshooting guide
- Improved error handling examples showing context-aware retry strategies
- Added WebSocket error handling to error_handling example (feature-gated)
- Refactored model organization to align with endpoint structure
  - Created new modules: `filings.rs` and `ownership.rs` for better organization
  - Moved models to match their endpoint usage (e.g., executive models to compliance)
  - Moved `IPOCalendar` and `IPOEvent` from stock models to calendar models
  - Moved `StatementType` and `StatementFrequency` from common to financials module
  - Removed obsolete modules: `alternative.rs`, `executive.rs`, and `fund.rs`

### Fixed
- Model parsing errors for optional fields in API responses
  - Made `ex_dividend_date` optional in `Dividend` model (some companies don't have ex-dividend dates)
  - Made `company_name` optional in `ESGScore` model (API sometimes omits company names)
  - Made `symbol` and `name` optional in `SupplyChainRelationship` model
  - Made `item` field optional in `SimilarityData` model
  - Fixed `EarningsQualityScore` response structure to match API format with wrapper object
  - Fixed `Dividend` model: changed `frequency` to `freq` field and made it optional
  - Fixed `BasicFinancials` series field: changed to `serde_json::Value` for complex nested structure
  - Fixed `SimilarityData` model: restructured to match actual API response with individual item scores
- Renamed `EarningsQualityScoreResponse` to `EarningsQualityScore` to follow codebase naming conventions
- Updated all imports and method signatures for consistency
- Fixed example code to handle optional fields in ESG data display

## [0.1.0] - 2025-05-28

### Added
- Initial release with 103/107 Finnhub API endpoints (96.3% coverage)
- Full async/await support with Tokio
- Type-safe request and response models
- Built-in rate limiting (30 requests/second)
- Basic WebSocket structure (feature-gated, not production-ready)
- Comprehensive error handling with `thiserror`
- Well-organized module structure by API categories

### API Coverage
- **Stock Market**: 52/54 endpoints
  - Quotes, company profiles, financials, earnings, dividends
  - Price targets, recommendations, insider transactions
  - ESG scores, patents, congressional trading, lobbying data
  - Technical indicators, market holidays, ownership data
- **Forex**: 4/4 endpoints (symbols, candles, rates, exchanges)
- **Crypto**: 4/4 endpoints (exchanges, symbols, candles, profile)
- **Bonds**: 4/4 endpoints (profile, price, tick, yield curve)
- **ETFs**: 4/4 endpoints (profile, holdings, exposures)
- **Mutual Funds**: 6/6 endpoints (profile, holdings, EET data)
- **Economic Data**: 2/2 endpoints (data, codes)
- **News**: 3/3 endpoints (market news, company news, sentiment)
- **Calendar**: 3/3 endpoints (earnings, economic, IPO)
- **Scanner**: 3/3 endpoints (patterns, support/resistance, indicators)
- **Misc**: 8/9 endpoints (search, countries, sector metrics, etc.)

### Known Limitations
- Missing POST request support (affects 1 endpoint: ai_chat)
- WebSocket implementation is minimal and not production-ready
- No automatic retry logic (by design - left to application layer)
- No response caching (by design - left to application layer)

[Unreleased]: https://github.com/jbradenbrown/finnhub/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jbradenbrown/finnhub/releases/tag/v0.1.0