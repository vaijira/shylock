use num_format::{Buffer, Locale};
use once_cell::sync::OnceCell;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use shylock_data::types::{Asset, Auction};
use shylock_data::{provinces::Province, BidInfo};
use std::collections::{BTreeSet, HashMap};
use std::{
    cmp::{max, min},
    ops::Add,
};
use substring::Substring;

pub static ASSETS: OnceCell<Vec<Asset>> = OnceCell::new();
pub static AUCTIONS: OnceCell<HashMap<String, Auction>> = OnceCell::new();
pub static MAX_AUCTION_VALUE: OnceCell<Decimal> = OnceCell::new();
pub static PROVINCES: OnceCell<BTreeSet<Province>> = OnceCell::new();
pub static CITIES: OnceCell<BTreeSet<&str>> = OnceCell::new();

pub const DESCRIPTION_TEXT_LIMIT: usize = 300;

pub fn get_bidinfo<'a>(bidinfo: &'a Option<BidInfo>, auction_id: &str) -> &'a BidInfo {
    if bidinfo.is_some() {
        bidinfo.as_ref().unwrap()
    } else {
        &AUCTIONS.get().unwrap().get(auction_id).unwrap().bidinfo
    }
}

pub fn format_valuation(valuation: &Decimal) -> String {
    let mut buf = Buffer::default();

    buf.write_formatted(&valuation.trunc().to_u64().unwrap_or(0), &Locale::es);

    format!(
        "{},{}",
        buf.as_str(),
        valuation.fract().to_u32().unwrap_or(0)
    )
}

pub fn summarize(text: &str) -> String {
    let str_min = min(text.chars().count(), DESCRIPTION_TEXT_LIMIT);
    let mut new_text = text.substring(0, str_min).to_string();

    if new_text.chars().count() == DESCRIPTION_TEXT_LIMIT {
        new_text = new_text.add("...");
    }

    new_text
}

pub(crate) fn set_global_info() {
    let max_auctions = AUCTIONS
        .get()
        .unwrap()
        .iter()
        .map(|(_, auction)| auction.bidinfo.value)
        .max()
        .unwrap();

    let max_assets = ASSETS
        .get()
        .unwrap()
        .iter()
        .filter(|asset| match asset {
            Asset::Property(property) => property.bidinfo.is_some(),
            Asset::Vehicle(vehicle) => vehicle.bidinfo.is_some(),
            Asset::Other(other) => other.bidinfo.is_some(),
        })
        .map(|asset| match asset {
            Asset::Property(property) => property.bidinfo.as_ref().unwrap().value,
            Asset::Vehicle(vehicle) => vehicle.bidinfo.as_ref().unwrap().value,
            Asset::Other(other) => other.bidinfo.as_ref().unwrap().value,
        })
        .max()
        .unwrap();

    if MAX_AUCTION_VALUE
        .set(max(max_assets, max_auctions))
        .is_err()
    {
        log::error!("Not able to set max auction value");
    };

    if PROVINCES
        .set(
            ASSETS
                .get()
                .unwrap()
                .iter()
                .filter_map(|asset| match asset {
                    Asset::Property(property) => Some(property.province),
                    Asset::Vehicle(_) => None,
                    Asset::Other(_) => None,
                })
                .collect::<BTreeSet<Province>>(),
        )
        .is_err()
    {
        log::error!("Not able to set provinces");
    };

    if CITIES
        .set(
            ASSETS
                .get()
                .unwrap()
                .iter()
                .filter_map(|asset| match asset {
                    Asset::Property(property) => Some(&property.city[..]),
                    Asset::Vehicle(_) => None,
                    Asset::Other(_) => None,
                })
                .collect::<BTreeSet<&str>>(),
        )
        .is_err()
    {
        log::error!("Not able to set cities");
    };
}
