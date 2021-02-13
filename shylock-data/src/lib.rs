#![warn(rust_2018_idioms, missing_docs, warnings, unused_extern_crates)]

//! Main data structures holding information about spanish auctions.

/// Auction concepts
pub mod concepts;

/// Spain provinces
pub mod provinces;

/// Auction types
pub mod types;

pub use self::types::*;
pub use chrono::NaiveDate;
pub use geo_types::Point;
pub use rust_decimal::Decimal;
