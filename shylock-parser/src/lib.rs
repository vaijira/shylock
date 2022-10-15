#![warn(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    unused_extern_crates,
    warnings
)]

//! Library to parse and collect information about spanish auctions.
//!
//! Current support:
//! * Extract information for https://subastas.boe.es
//!

#[macro_use]
extern crate lazy_static;

/// Module for accessing the auction local database.
pub mod db;
/// Module for solving address into coordinates.
pub mod geosolver;
/// Module communicating through http to BOE website
pub mod http;
/// Module to parse HTML BOE pages
pub mod parser;
/// Module to browse BOE website.
pub mod scraper;

/// Module with auxiliary functions.
pub mod util;

pub use chrono::NaiveDate;
pub use geo_types::Point;
pub use rust_decimal::Decimal;
pub use shylock_data::types::*;
