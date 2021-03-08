use crate::global::AUCTIONS;
use num_format::{Buffer, Locale};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use shylock_data::BidInfo;
use std::{cmp::min, ops::Add};
use substring::Substring;

pub const DESCRIPTION_TEXT_LIMIT: usize = 300;

pub const SUBASTAS_BOE_URI: &str = "https://subastas.boe.es/detalleSubasta.php?idSub=";

pub fn get_external_url(auction_id: &str) -> String {
    format!("{}{}", SUBASTAS_BOE_URI, auction_id)
}

pub fn is_targeted_asset(bidinfo: &Option<BidInfo>, auction_id: &str) -> bool {
    let bidinfo = get_bidinfo(bidinfo, auction_id);
    let target_value = &bidinfo.value.to_f64().or(Some(0.0)).unwrap() * 0.7;

    target_value > bidinfo.claim_quantity.to_f64().or(Some(0.0)).unwrap()
}

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
