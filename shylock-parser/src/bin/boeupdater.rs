use std::collections::BTreeMap;

use clap::{arg, Command};
use env_logger::Env;
use futures::{stream, StreamExt};
use shylock_data::types::{Asset, Auction};
use shylock_parser::{
    db::{DbClient, DEFAULT_DB_PATH},
    geosolver::GeoSolver,
    http::{UrlFetcher, MAIN_ALL_AUCTIONS_BOE_URL},
    image::create_svg_histogram,
    scraper::{auction_state_page_scraper, page_scraper, DEFAULT_COUNTRY},
    util::{dump_to_cbor_compressed_file, valid_catastro_reference},
    AuctionState,
};

const DEFAULT_CONCURRENCY: usize = 6;

async fn init_scrape(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = &UrlFetcher::new();
    let main_page = http_client.get_url(&MAIN_ALL_AUCTIONS_BOE_URL).await?;
    let mut pages_url = shylock_parser::parser::parse_extra_pages(&main_page);

    pages_url.insert(0, MAIN_ALL_AUCTIONS_BOE_URL.to_string());
    log::info!("Total BOE pages to process: {}", pages_url.len());

    let stream = stream::iter(pages_url.iter().enumerate());

    stream
        .for_each_concurrent(DEFAULT_CONCURRENCY, |page| async move {
            if let Ok((ok, err, already_proccessed)) =
                page_scraper(http_client, db_client, page.1).await
            {
                log::info!(
                    "Page {} ended succesfully ok {}/err {}/total {}",
                    page.0,
                    ok,
                    err,
                    already_proccessed
                );
            }
        })
        .await;

    Ok(())
}

async fn update_scrape(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    let auction_states = &[
        AuctionState::Ongoing,
        AuctionState::ToBeOpened,
        AuctionState::Suspended,
    ];
    let http_client = &UrlFetcher::new();
    let auction_ids = &db_client
        .get_auction_ids_with_states(auction_states)
        .await?;

    log::info!(
        "Total BOE ongoing auctions to check for current state: {}",
        auction_ids.len()
    );
    let main_page = http_client.get_url(&MAIN_ALL_AUCTIONS_BOE_URL).await?;
    let mut pages_url = shylock_parser::parser::parse_extra_pages(&main_page);

    pages_url.insert(0, MAIN_ALL_AUCTIONS_BOE_URL.to_string());
    log::info!("Total BOE pages to process: {}", pages_url.len());

    let stream = stream::iter(pages_url.iter().enumerate());

    stream
        .for_each_concurrent(DEFAULT_CONCURRENCY, |page| async move {
            if let Ok(ok) =
                auction_state_page_scraper(http_client, db_client, auction_ids, page.1).await
            {
                log::info!("Update auctions: {} for page {}.", ok, page.0,);
            }
        })
        .await;

    Ok(())
}

async fn export_ongoing_auctions(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut auctions: BTreeMap<String, Auction> = BTreeMap::new();
    let mut assets: Vec<Asset> = Vec::new();
    let geosolver = &GeoSolver::new();

    db_client
        .get_auctions_with_states(&[AuctionState::Ongoing])
        .await?
        .into_iter()
        .for_each(|x| {
            auctions.insert(x.id.clone(), x);
        });

    let auction_file = format!(
        "{}/../shylock-dominator/{}",
        env!("CARGO_MANIFEST_DIR"),
        "auctions.cbor.zlib"
    );
    dump_to_cbor_compressed_file(&auction_file, &auctions)?;

    let mut properties = db_client
        .get_properties_with_auction_states(&[AuctionState::Ongoing])
        .await?;

    stream::iter(properties.iter_mut())
        .for_each_concurrent(DEFAULT_CONCURRENCY, |property| async move {
            if property.catastro_link == None
                && valid_catastro_reference(&property.catastro_reference)
            {
                match geosolver
                    .get_catastro_link(&property.catastro_reference)
                    .await
                {
                    Ok(link) => property.catastro_link = link,
                    Err(error) => log::warn!(
                        "Unable to get catastro link {} for catastro reference: {}",
                        error,
                        &property.catastro_reference
                    ),
                }
            }
            if property.coordinates == None {
                property.coordinates = match geosolver
                    .resolve(
                        &property.address,
                        &property.city,
                        property.province.name(),
                        DEFAULT_COUNTRY,
                        &property.postal_code,
                        &property.catastro_reference,
                    )
                    .await
                {
                    Ok(coordinates) => coordinates,
                    Err(error) => {
                        log::warn!("Unable to retrieve coordinates: {}", error);
                        None
                    }
                };
            }
        })
        .await;

    properties
        .into_iter()
        .for_each(|property| assets.push(Asset::Property(property)));

    stream::iter(assets.iter())
        .for_each_concurrent(1, |asset| async move {
            if let Asset::Property(property) = asset {
                if property.coordinates.is_some() {
                    if let Err(err) = db_client.update_asset_coordinate(property).await {
                        log::warn!("Unable to update coordinates: {}", err);
                    }
                }
            }
        })
        .await;

    db_client
        .get_vehicles_with_auction_states(&[AuctionState::Ongoing])
        .await?
        .into_iter()
        .for_each(|x| {
            assets.push(Asset::Vehicle(x));
        });

    db_client
        .get_other_assets_with_auction_states(&[AuctionState::Ongoing])
        .await?
        .into_iter()
        .for_each(|x| {
            assets.push(Asset::Other(x));
        });

    let assets_file = format!(
        "{}/../shylock-dominator/{}",
        env!("CARGO_MANIFEST_DIR"),
        "assets.cbor.zlib"
    );
    dump_to_cbor_compressed_file(&assets_file, &assets)?;

    Ok(())
}

async fn export_auction_statistics(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    let data = db_client.get_auctions_by_month_statistics().await?;

    let out_file_path = format!(
        "{}/../shylock-dominator/dist/images/{}",
        env!("CARGO_MANIFEST_DIR"),
        "auctions_by_month.svg"
    );
    create_svg_histogram(&data[1..], &out_file_path)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let matches = Command::new("boeupdater")
        .version("0.1")
        .author("Jorge Perez Burgos <vaijira@gmail.com>")
        .about("Update db with latest auctions BOE information.")
        .arg(
            arg!(<MODE>)
                .help(
                    r#"create: creates database and tables.
init: initializes database loading all auctions and assets.
update: updates ongoing auctions status.
export: exports ongoing auctions and assets to cbor files.
statistics: exports auction statistics as images.
"#,
                )
                .value_parser(["create", "init", "update", "export", "statistics"]),
        )
        .arg(
            arg!(-d --db_path <DB_PATH> "Sets the database path, default: ./db/shylock.db")
                .required(false),
        )
        .get_matches();

    let db_path = matches.value_of("db_path").unwrap_or(DEFAULT_DB_PATH);

    let db_client = DbClient::new(db_path).await?;

    match matches
        .get_one::<String>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
        .as_str()
    {
        "create" => {
            log::info!("Createing database and tables.");
            db_client.migrate().await?;
        }
        "init" => {
            log::info!("Initialization mode going to all auctions.");
            let _ = init_scrape(&db_client).await;
        }
        "update" => {
            log::info!("Updating status of ongoing auctions.");
            let _ = update_scrape(&db_client).await;
        }
        "export" => {
            log::info!("Exporting ongoing auctions and assets to cbor files.");
            let _ = export_ongoing_auctions(&db_client).await;
        }
        "statistics" => {
            log::info!("Exporting auction statistics as images.");
            let _ = export_auction_statistics(&db_client).await;
        }
        _ => unreachable!(),
    }

    Ok(())
}
