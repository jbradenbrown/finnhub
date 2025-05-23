//! API endpoint implementations.

pub mod stock;
pub mod forex;
pub mod crypto;

pub use stock::StockEndpoints;
pub use forex::ForexEndpoints;
pub use crypto::CryptoEndpoints;