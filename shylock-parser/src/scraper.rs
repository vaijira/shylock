use crate::db::DbClient;
use crate::http::UrlFetcher;
use crate::parser::*;
use crate::util::{extract_auction_id_from_link, extract_auction_lot_number_from_link};
use shylock_data::types::{Asset, Auction, LotAuctionKind, Management};
use shylock_data::AuctionState;

/// Default country to solve geographic information.
pub const DEFAULT_COUNTRY: &str = "Spain";

/// Retrieve an auction information from an auction link.
pub async fn process_auction_link(
    url_fetcher: &UrlFetcher,
    link: &(String, AuctionState),
) -> Result<(Auction, Vec<Asset>), Box<dyn std::error::Error>> {
    let mut assets = Vec::new();

    let auction_page = url_fetcher.get_url(&link.0).await?;

    let (mgm_link, asset_link) = parse_main_auction_links(&auction_page)?;
    let management_page = url_fetcher.get_url(&mgm_link).await?;
    let management = Management::new(&parse_management_auction_page(&management_page)?);
    log::info!("Created management: {}", management.code);

    let auction = Auction::new(&parse_main_auction_page(&auction_page)?, management, link.1);
    log::info!("Created auction: {}", auction.id);

    let asset_page = url_fetcher.get_url(&asset_link).await?;
    match auction.lot_kind {
        LotAuctionKind::NotApplicable => {
            log::info!("Parsing auction without lots link");
            let asset = Asset::new(&auction.id, &parse_asset_auction_page(&asset_page)?);

            assets.push(asset);
        }
        LotAuctionKind::Joined | LotAuctionKind::Splitted => {
            let lot_links = parse_lot_auction_page_links(&asset_page)?;
            for lot_link in lot_links.iter() {
                log::info!("Visiting lot auction link: {}", lot_link);
                let lot_page = url_fetcher.get_url(lot_link).await?;

                let lot_id = extract_auction_lot_number_from_link(lot_link)?;

                let asset = Asset::new(&auction.id, &parse_lot_auction_page(&lot_page, lot_id)?);
                assets.push(asset);
            }
        }
    }

    Ok((auction, assets))
}

/// Scrape all links of a page.
pub async fn page_scraper(
    http_client: &UrlFetcher,
    db_client: &DbClient,
    result_page_url: &str,
) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    let mut auction_ok: u32 = 0;
    let mut auction_err: u32 = 0;
    let mut auction_already_process: u32 = 0;

    log::info!("page url to process: {}", result_page_url);
    let result_page = http_client.get_url(result_page_url).await?;
    let auction_links = parse_result_page(&result_page);
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

/// Scrape auction page
pub async fn auction_state_page_scraper(
    http_client: &UrlFetcher,
    db_client: &DbClient,
    auction_ids: &[String],
    result_page_url: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut auction_ok: u32 = 0;

    log::info!("page url to process: {}", result_page_url);
    let result_page = http_client.get_url(result_page_url).await?;
    let auction_links = parse_result_page(&result_page);

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
