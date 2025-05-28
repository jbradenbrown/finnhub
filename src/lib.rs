//! # Finnhub Rust Client
//!
//! A comprehensive, type-safe Rust client for the Finnhub.io financial data API.
//!
//! ## Features
//!
//! - Full API coverage with strongly typed requests and responses
//! - Async/await support via Tokio
//! - Built-in rate limiting (30 requests/second)
//! - WebSocket support for real-time data
//! - Comprehensive error handling
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
//!     let quote = client.quote("AAPL").await?;
//!     println!("AAPL price: ${}", quote.current_price);
//!     
//!     Ok(())
//! }
//! ```

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

pub use client::FinnhubClient;
pub use error::{Error, Result};

#[doc(hidden)]
pub mod prelude {
    pub use crate::client::FinnhubClient;
    pub use crate::error::{Error, Result};
}
