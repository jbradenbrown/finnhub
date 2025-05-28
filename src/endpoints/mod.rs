//! API endpoint implementations.

pub mod crypto;
pub mod forex;
pub mod stock;

pub use crypto::CryptoEndpoints;
pub use forex::ForexEndpoints;
pub use stock::StockEndpoints;
