use once_cell::sync::OnceCell;
use rust_decimal::Decimal;
use shylock_data::provinces::Province;
use shylock_data::types::{Asset, Auction};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

pub static ASSETS: OnceCell<Vec<Asset>> = OnceCell::new();
pub static AUCTIONS: OnceCell<HashMap<String, Auction>> = OnceCell::new();
pub static MAX_AUCTION_VALUE: OnceCell<Decimal> = OnceCell::new();
pub static PROVINCES: OnceCell<BTreeSet<Province>> = OnceCell::new();
pub static CITIES: OnceCell<BTreeSet<&str>> = OnceCell::new();

pub const DEFAULT_UX_ASSET_SIZE: &str = "25";
pub const DEFAULT_UX_ASSET_COLOR: &str = "#FFFFFF";

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
