#![warn(rust_2018_idioms, missing_docs, warnings)]

//! Library to parse and collect information about spanish auctions.
//!
//! Current support:
//! * Extract information for https://subastas.boe.es
//!

#[macro_use]
extern crate lazy_static;

mod concepts;
mod geosolver;
pub(crate) mod parser;
mod scraper;
mod types;

pub use self::scraper::scrape;
pub use self::types::*;
pub use chrono::NaiveDate;
pub use geo_types::Point;
pub use rust_decimal::Decimal;
