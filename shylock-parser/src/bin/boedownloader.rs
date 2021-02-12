use clap::clap_app;
use env_logger::Env;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::str;

const AUCTION_DATA_JSON_FILE_NAME: &str = "auctions.json";
const ASSETS_DATA_JSON_FILE_NAME: &str = "assets.json";

fn dump_to_json_file<T>(dst_path: &str, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(&data).unwrap();

    let mut dest = {
        log::info!("data json file will be located under: '{:?}'", dst_path);
        File::create(dst_path)?
    };

    io::copy(&mut json.as_bytes(), &mut dest)?;
    log::info!("data json file created");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Jorge Perez Burgos <vaijira@gmail.com>")
        (about: "Download subastas.boe.es ongoing auctions.")
        (@arg OUTPUT_DIR: -o --output +takes_value "Sets the output directory, default: /tmp")
    )
    .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let output_dir = matches.value_of("OUTPUT_DIR").unwrap_or("/tmp");
    log::info!("Value for output dir: {}", output_dir);

    let (auctions, assets) = shylock_parser::scrape()?;

    let auction_json_file = format!("{}/{}", output_dir, AUCTION_DATA_JSON_FILE_NAME);
    dump_to_json_file(&auction_json_file, &auctions)?;

    let assets_json_file = format!("{}/{}", output_dir, ASSETS_DATA_JSON_FILE_NAME);
    dump_to_json_file(&assets_json_file, &assets)?;

    Ok(())
}
