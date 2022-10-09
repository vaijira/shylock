use std::collections::BTreeMap;

use clap::{arg, Command};
use env_logger::Env;
use futures::{stream, StreamExt};
use shylock_data::types::{Asset, Auction, LotAuctionKind, Management};
use shylock_parser::{
    db::{DbClient, DEFAULT_DB_PATH},
    geosolver::GeoSolver,
    http::{UrlFetcher, MAIN_ALL_AUCTIONS_BOE_URL},
    scraper::DEFAULT_COUNTRY,
    util::{dump_to_cbor_file, extract_auction_id_from_link, extract_auction_lot_number_from_link},
    AuctionState,
};

async fn process_auction_link(
    url_fetcher: &UrlFetcher,
    link: &(String, AuctionState),
) -> Result<(Auction, Vec<Asset>), Box<dyn std::error::Error>> {
    let mut assets = Vec::new();

    let auction_page = url_fetcher.get_url(&link.0).await?;

    let (mgm_link, asset_link) = shylock_parser::parser::parse_main_auction_links(&auction_page)?;
    let management_page = url_fetcher.get_url(&mgm_link).await?;
    let management = Management::new(&shylock_parser::parser::parse_management_auction_page(
        &management_page,
    )?);
    log::info!("Created management: {}", management.code);

    let auction = Auction::new(
        &shylock_parser::parser::parse_main_auction_page(&auction_page)?,
        management,
        link.1,
    );
    log::info!("Created auction: {}", auction.id);

    let asset_page = url_fetcher.get_url(&asset_link).await?;
    match auction.lot_kind {
        LotAuctionKind::NotApplicable => {
            let asset = Asset::new(
                &auction.id,
                &shylock_parser::parser::parse_asset_auction_page(&asset_page)?,
            );

            assets.push(asset);
        }
        LotAuctionKind::Joined | LotAuctionKind::Splitted => {
            let lot_links = shylock_parser::parser::parse_lot_auction_page_links(&asset_page)?;
            for lot_link in lot_links.iter() {
                let lot_page = url_fetcher.get_url(lot_link).await?;

                let lot_id = extract_auction_lot_number_from_link(lot_link)?;

                let asset = Asset::new(
                    &auction.id,
                    &shylock_parser::parser::parse_lot_auction_page(&lot_page, lot_id)?,
                );
                assets.push(asset);
            }
        }
    }

    Ok((auction, assets))
}

async fn page_scraper(
    http_client: &UrlFetcher,
    db_client: &DbClient,
    result_page_url: &str,
) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    let mut auction_ok: u32 = 0;
    let mut auction_err: u32 = 0;
    let mut auction_already_process: u32 = 0;

    log::info!("page url to process: {}", result_page_url);
    let result_page = http_client.get_url(result_page_url).await?;
    let auction_links = shylock_parser::parser::parse_result_page(&result_page);
    log::info!("processing {} links", auction_links.len());
    let number_auctions = auction_links.len();

    for auction_link in auction_links {
        let auction_id = extract_auction_id_from_link(&auction_link.0)?;

        if let Ok(true) = db_client.auction_exists(auction_id).await {
            log::info!("Auction ->{}<- previously processed", auction_id);
            auction_already_process += 1;
            continue;
        }
        match process_auction_link(http_client, &auction_link).await {
            Ok((auction, auction_assets)) => {
                let tx = db_client.pool.begin().await?;

                db_client.insert_management(&auction.management).await;

                db_client.insert_auction(&auction).await;

                db_client.insert_assets(&auction, &auction_assets).await;

                tx.commit().await?;

                auction_ok += 1;
            }
            Err(err) => {
                auction_err += 1;
                log::warn!("Unable to process: {}", err)
            }
        }
        log::info!(
            "Auctions processed: {}/{}, Auctions errors: {}, previously processed: {}",
            auction_ok + auction_err + auction_already_process,
            number_auctions,
            auction_err,
            auction_already_process
        );
    }

    Ok((auction_ok, auction_err, auction_already_process))
}

async fn init_scrape(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = &UrlFetcher::new();
    let main_page = http_client.get_url(&MAIN_ALL_AUCTIONS_BOE_URL).await?;
    let mut pages_url = shylock_parser::parser::parse_extra_pages(&main_page);

    pages_url.insert(0, MAIN_ALL_AUCTIONS_BOE_URL.to_string());
    log::info!("Total BOE pages to process: {}", pages_url.len());

    let stream = stream::iter(pages_url.iter().enumerate());

    stream
        .for_each_concurrent(6, |page| async move {
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

async fn auction_state_page_scraper(
    http_client: &UrlFetcher,
    db_client: &DbClient,
    auction_ids: &[String],
    result_page_url: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut auction_ok: u32 = 0;

    log::info!("page url to process: {}", result_page_url);
    let result_page = http_client.get_url(result_page_url).await?;
    let auction_links = shylock_parser::parser::parse_result_page(&result_page);

    for auction_link in auction_links {
        let auction_id = extract_auction_id_from_link(&auction_link.0)?;

        if auction_ids.iter().any(|s| s == auction_id) {
            db_client
                .update_auction_state(auction_id, auction_link.1)
                .await?;
            log::info!("Updated state of auction ->{}<-", auction_id);
            auction_ok += 1;
        }
    }

    Ok(auction_ok)
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
        .for_each_concurrent(6, |page| async move {
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

    let auction_file = format!("{}/../shylock-dominator/{}", env!("CARGO_MANIFEST_DIR"), "auctions.cbor");
    dump_to_cbor_file(&auction_file, &auctions)?;

    let mut properties = db_client
        .get_properties_with_auction_states(&[AuctionState::Ongoing])
        .await?;

    stream::iter(properties.iter_mut())
        .for_each(|property| async move {
            if property.coordinates == None {
                property.coordinates = match geosolver
                    .resolve(
                        &property.address,
                        &property.city,
                        property.province.name(),
                        DEFAULT_COUNTRY,
                        &property.postal_code,
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

    let assets_file = format!("{}/../shylock-dominator/{}", env!("CARGO_MANIFEST_DIR"), "assets.cbor");
    dump_to_cbor_file(&assets_file, &assets)?;

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
                    r#"init: initialize database loading all auctions and assets.
update: update ongoing auctions status.
export: export ongoing auctions and assets to json files.
"#,
                )
                .value_parser(["init", "update", "export"]),
        )
        .arg(
            arg!(-d --db_path <DB_PATH> "Sets the database path, default: ./db/shylock.db")
                .required(false),
        )
        .get_matches();

    let db_path = matches.value_of("db_path").unwrap_or(DEFAULT_DB_PATH);

    let db_client = DbClient::new(db_path).await?;

    db_client.migrate().await?;

    match matches
        .get_one::<String>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
        .as_str()
    {
        "init" => {
            log::info!("Initialization mode going to all auctions.");
            let _ = init_scrape(&db_client).await;
        }
        "update" => {
            log::info!("Updating status of ongoing auctions.");
            let _ = update_scrape(&db_client).await;
        }
        "export" => {
            log::info!("Exporting ongoing auctions and assets to json files.");
            let _ = export_ongoing_auctions(&db_client).await;
        }
        _ => unreachable!(),
    }

    Ok(())
}
