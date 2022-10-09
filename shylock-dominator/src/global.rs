use dominator::{class, pseudo};
use once_cell::sync::{Lazy, OnceCell};
use rust_decimal::Decimal;
use shylock_data::provinces::Province;
use shylock_data::types::{Asset, Auction};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap};
use wasm_bindgen::JsValue;

pub static ASSETS: OnceCell<Vec<Asset>> = OnceCell::new();
pub static AUCTIONS: OnceCell<HashMap<String, Auction>> = OnceCell::new();
pub static MAX_AUCTION_VALUE: OnceCell<Decimal> = OnceCell::new();
pub static PROVINCES: OnceCell<BTreeSet<Province>> = OnceCell::new();
pub static CITIES_PROVINCES: OnceCell<BTreeSet<(&str, Province)>> = OnceCell::new();

pub const DEFAULT_ICON_COLOR: &str = "black";

pub static ROOT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("padding", "10px")
    }
});

pub static SECTION_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "inline-block")
        .style("overflow", "auto")
        .style("width", "100%")
        .style("height", "calc(100vh - 118px)")
    }
});

pub static TABLE_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("border", "1px solid")
        .style("border-collapse", "collapse")
    }
});

pub static THEAD_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("text-align", "left")
        .style("position", "sticky")
    }
});

pub static TBODY_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("text-align", "left")
    }
});

pub static ROW_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("height", "3em")
        .pseudo!(":hover", {
            .style("background-color", "coral")
        })
        /*.pseudo!(":nth-child(even)", {
            .style("background-color", "lightgray")
        })*/
    }
});

pub static CELL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("border-bottom", "1px solid")
    }
});

pub static CELL_EXPANDED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("height", "10em")
        .style("border-bottom", "1px solid")
    }
});

pub static CELL_FLEX_CONTAINER_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-flow", "row wrap")
        .style("gap", "10px 10px")
    }
});

pub static CELL_FLEX_ITEM_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("flex", "1 100%")
    }
});

pub static FILTER_FLEX_CONTAINER_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-flow", "row wrap")
        .style("gap", "10px 10px")
    }
});

pub static NAVBAR_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("border-bottom", "1px solid black")
    }
});

pub static NAV_UL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("list-style-type", "none")
        .style("margin", "0")
        .style("padding", "0")
        .style("overflow", "hidden")
        .style("background-color", "white")
    }
});

pub static NAVITEM_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("float", "left")
    }
});

pub static NAV_LINK_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "block")
        .style("color", "black")
        .style("text-align", "center")
        .style("padding", "14px 16px")
        .style("text-decoration", "none")
        .pseudo!(":hover", {
            .style("background-color", "gray")
        })
    }
});

pub static NAV_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "block")
        .style("color", "black")
        .style("background-color", "lightgray")
        .style("text-align", "center")
        .style("padding", "14px 16px")
        .style("text-decoration", "none")
        .style("pointer-events", "none")
    }
});

pub(crate) async fn set_global_info() -> Result<(), JsValue> {
    let hm: HashMap<String, Auction> =
        ciborium::de::from_reader(include_bytes!("../auctions.cbor").as_slice()).unwrap();

    if AUCTIONS.set(hm).is_err() {
        log::error!("Unable to set global auctions");
    }

    let assets: Vec<Asset> =
        ciborium::de::from_reader(include_bytes!("../assets.cbor").as_slice()).unwrap();

    if ASSETS.set(assets).is_err() {
        log::error!("Unable to set global assets");
    }

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

    if CITIES_PROVINCES
        .set(
            ASSETS
                .get()
                .unwrap()
                .iter()
                .filter_map(|asset| match asset {
                    Asset::Property(property) => Some((&property.city[..], property.province)),
                    Asset::Vehicle(_) => None,
                    Asset::Other(_) => None,
                })
                .collect::<BTreeSet<(&str, Province)>>(),
        )
        .is_err()
    {
        log::error!("Not able to set cities");
    };

    Ok(())
}
