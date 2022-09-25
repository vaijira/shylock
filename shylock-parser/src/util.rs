// Constant for parsing lot number.
const AUCTION_LOT_NUMBER_STR: &str = "idLote=";

// Constant for parsing auction identifier.
const AUCTION_ID_LINK_STR: &str = "?idSub=";

fn extract_field_value_from_link<'l>(
    link: &'l str,
    tag: &str,
) -> Result<&'l str, Box<dyn std::error::Error>> {
    let id_begin = link.find(tag).unwrap() + tag.len();
    let id_end = link[id_begin..].find('&').unwrap() + id_begin;
    Ok(&link[id_begin..id_end])
}
/// Extract auction id from a `link`.
pub fn extract_auction_id_from_link(link: &str) -> Result<&str, Box<dyn std::error::Error>> {
    extract_field_value_from_link(link, AUCTION_ID_LINK_STR)
}

/// Extract auction lot number from a `link`.
pub fn extract_auction_lot_number_from_link(
    link: &str,
) -> Result<&str, Box<dyn std::error::Error>> {
    extract_field_value_from_link(link, AUCTION_LOT_NUMBER_STR)
}
