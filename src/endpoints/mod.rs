//! API endpoint implementations.

pub mod bond;
pub mod calendar;
pub mod crypto;
pub mod economic;
pub mod etf;
pub mod forex;
pub mod index;
pub mod misc;
pub mod mutual_fund;
pub mod news;
pub mod scanner;
pub mod stock;

pub use bond::BondEndpoints;
pub use calendar::CalendarEndpoints;
pub use crypto::CryptoEndpoints;
pub use economic::EconomicEndpoints;
pub use etf::ETFEndpoints;
pub use forex::ForexEndpoints;
pub use index::IndexEndpoints;
pub use misc::MiscEndpoints;
pub use mutual_fund::MutualFundEndpoints;
pub use news::NewsEndpoints;
pub use scanner::ScannerEndpoints;
pub use stock::StockEndpoints;
