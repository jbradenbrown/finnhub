//! # Finnhub Rust Client
//!
//! A comprehensive, type-safe Rust client for the Finnhub.io financial data API.
//!
//! ## Features
//!
//! - 📊 Extensive API coverage (103/107 endpoints - 96.3%)
//! - 🚀 Full async/await support via Tokio
//! - ⚡ Built-in rate limiting with flexible strategies
//! - 🔄 Basic WebSocket structure (feature-gated, not production-ready)
//! - 🛡️ Comprehensive error handling with retry helpers
//! - 🔒 Type-safe request and response models
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use finnhub::FinnhubClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), finnhub::Error> {
//!     let client = FinnhubClient::new("your-api-key");
//!     
//!     // Get a stock quote
//!     let quote = client.stock().quote("AAPL").await?;
//!     println!("AAPL price: ${}", quote.current_price);
//!     
//!     // Get company profile
//!     let profile = client.stock().company_profile("AAPL").await?;
//!     println!("Company: {}", profile.name.unwrap_or_default());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Design Philosophy
//!
//! This library follows a minimalist design philosophy:
//! - **No automatic retries**: Applications implement context-aware retry logic
//! - **No response caching**: Applications manage cache based on their needs
//! - **Flexible rate limiting**: Choose between strict per-second or burst-friendly strategies
//!
//! The library provides the tools (`is_retryable()`, `retry_after()`) but lets you decide how to use them.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod rate_limiter;

#[cfg(feature = "websocket")]
pub mod websocket;

pub use client::{ClientConfig, FinnhubClient, RateLimitStrategy};
pub use error::{Error, Result};
pub use rate_limiter::RateLimiter;

#[doc(hidden)]
pub mod prelude {
    pub use crate::client::FinnhubClient;
    pub use crate::error::{Error, Result};
}
