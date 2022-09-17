use clap::{arg, Command};
use env_logger::Env;
use futures::*;
use shylock_data::types::{Asset, Auction, LotAuctionKind, Management};
use shylock_parser::{
    db::{DbClient, DEFAULT_DB_PATH},
    http::{UrlFetcher, MAIN_ALL_AUCTIONS_BOE_URL},
    scraper::AUCTION_LOT_NUMBER_STR,
    AuctionState,
};

const AUCTION_ID_LINK_STR: &str = "?idSub=";

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

                let lot_id_begin =
                    lot_link.find(AUCTION_LOT_NUMBER_STR).unwrap() + AUCTION_LOT_NUMBER_STR.len();
                let lot_id_end = lot_link[lot_id_begin..].find('&').unwrap() + lot_id_begin;
                let lot_id = &lot_link[lot_id_begin..lot_id_end];

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
        let id_begin =
            auction_link.0.find(AUCTION_ID_LINK_STR).unwrap() + AUCTION_ID_LINK_STR.len();
        let id_end = auction_link.0[id_begin..].find('&').unwrap() + id_begin;
        let auction_id = &auction_link.0[id_begin..id_end];

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

async fn scrape(db_client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let matches = Command::new("boeupdater")
        .version("0.1")
        .author("Jorge Perez Burgos <vaijira@gmail.com>")
        .about("Update db with latest auctions BOE information.")
        .arg(
            arg!(-d --db_path <DB_PATH> "Sets the database path, default: ./db/shylock.db")
                .required(false),
        )
        .get_matches();

    let db_path = matches.value_of("db_path").unwrap_or(DEFAULT_DB_PATH);

    let db_client = DbClient::new(db_path).await?;

    db_client.migrate().await?;

    let _ = scrape(&db_client).await;

    Ok(())
}
