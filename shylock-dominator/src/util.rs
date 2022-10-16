use dominator::Dom;
use js_sys::Error;
use num_format::{Buffer, Locale};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use shylock_data::BidInfo;
use std::cmp::min;
use substring::Substring;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, RequestInit, Response};

use crate::feather::render_svg_crosshair_icon;
use crate::global::DEFAULT_ICON_COLOR;

pub const DESCRIPTION_TEXT_LIMIT: usize = 150;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingOrder {
    None,
    Up,
    Down,
}

pub fn is_targeted_asset(bidinfo: &BidInfo) -> Vec<Dom> {
    let target_value = &bidinfo.value.to_f64().unwrap_or(0.0) * 0.7;

    if bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0
        && target_value > bidinfo.claim_quantity.to_f64().unwrap_or(0.0)
    {
        vec![render_svg_crosshair_icon(DEFAULT_ICON_COLOR, "12")]
    } else {
        vec![]
    }
}

pub async fn _fetch_json(url: &str) -> Result<String, JsValue> {
    let headers = Headers::new()?;

    let future = window()
        .unwrap()
        .fetch_with_str_and_init(url, RequestInit::new().headers(&headers));

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    if !response.ok() {
        return Err(Error::new("Fetch failed").into());
    }

    let value = JsFuture::from(response.text()?).await?.as_string().unwrap();

    Ok(value)
}

pub fn summarize(text: &str) -> &str {
    let str_min = min(text.chars().count(), DESCRIPTION_TEXT_LIMIT);
    text.substring(0, str_min)
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

pub fn new_bidinfo(bidinfo: &BidInfo, auction_bidinfo: &BidInfo) -> BidInfo {
    BidInfo {
        appraisal: if bidinfo.appraisal.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.appraisal
        } else {
            auction_bidinfo.appraisal
        },
        bid_step: if bidinfo.bid_step.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.bid_step
        } else {
            auction_bidinfo.bid_step
        },
        claim_quantity: if bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.claim_quantity
        } else {
            auction_bidinfo.claim_quantity
        },
        deposit: if bidinfo.deposit.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.deposit
        } else {
            auction_bidinfo.deposit
        },
        minimum_bid: if bidinfo.minimum_bid.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.minimum_bid
        } else {
            auction_bidinfo.minimum_bid
        },
        value: if bidinfo.value.to_f64().unwrap_or(0.0) > 1.0 {
            bidinfo.value
        } else {
            auction_bidinfo.value
        },
    }
}
