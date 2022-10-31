use miniz_oxide::deflate::compress_to_vec;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;
use std::str;

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

/// Serialize  `data` to json files in given `dst_path`.
pub fn dump_to_json_file<T>(dst_path: &str, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let json = serde_json::to_string(&data).unwrap();

    let mut dest = {
        log::info!("data json file will be located under: '{:?}'", dst_path);
        File::create(dst_path)?
    };

    io::copy(&mut json.as_bytes(), &mut dest)?;
    log::info!("data json file created");

    Ok(())
}

/// Serialize  `data` to messagepack files in given `dst_path`.
pub fn dump_to_rmp_file<T>(dst_path: &str, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let rmp = rmp_serde::to_vec(&data).unwrap();

    let mut dest = {
        log::info!("data rmp file will be located under: '{:?}'", dst_path);
        File::create(dst_path)?
    };

    io::copy(&mut &rmp[..], &mut dest)?;
    log::info!("data rmp file created");

    Ok(())
}

/// Serialize  `data` to cbor files in given `dst_path`.
pub fn dump_to_cbor_compressed_file<T>(
    dst_path: &str,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let mut tmp_buffer = vec![];
    ciborium::ser::into_writer(&data, &mut tmp_buffer)?;

    let mut file = {
        log::info!("data cbor file will be located under: '{:?}'", dst_path);
        File::create(dst_path)?
    };

    let compressed = compress_to_vec(&tmp_buffer, 7);

    file.write_all(compressed.as_slice())?;

    log::info!("data cbor file created");

    Ok(())
}

/// Normalize string
pub fn normalize(str: &str) -> String {
    str.to_uppercase()
        .chars()
        .filter_map(|x| match x {
            'Á' => Some('A'),
            'É' => Some('E'),
            'Í' => Some('I'),
            'Ó' => Some('O'),
            'Ú' => Some('U'),
            '.' => None,
            _ => Some(x),
        })
        .collect()
}

/// Check if string it's a valid catastro reference
pub fn valid_catastro_reference(reference: &str) -> bool {
    reference.len() == 20 && !reference.contains(' ')
}
