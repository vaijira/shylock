use clap::{arg, Command};
use env_logger::Env;
use shylock_parser::util::dump_to_json_file;

const AUCTION_DATA_JSON_FILE_NAME: &str = "auctions.json";
const ASSETS_DATA_JSON_FILE_NAME: &str = "assets.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let matches = Command::new("boedownloader")
        .version("0.1")
        .author("Jorge Perez Burgos <vaijira@gmail.com>")
        .about("Download subastas.boe.es ongoing auctions.")
        .arg(arg!(-o --output <OUTPUT_DIR> "Sets the output directory, default: /tmp"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let output_dir = matches.value_of("output").unwrap_or("/tmp");
    log::info!("Value for output dir: {}", output_dir);

    let (auctions, assets) = shylock_parser::scrape()?;

    let auction_json_file = format!("{}/{}", output_dir, AUCTION_DATA_JSON_FILE_NAME);
    dump_to_json_file(&auction_json_file, &auctions)?;

    let assets_json_file = format!("{}/{}", output_dir, ASSETS_DATA_JSON_FILE_NAME);
    dump_to_json_file(&assets_json_file, &assets)?;

    Ok(())
}
