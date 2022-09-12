use crate::http::{BlockingUrlFetcher, HttpClient, MAIN_ONGOING_AUCTIONS_BOE_URL};
use crate::{geosolver::GeoSolver, parser::*, AuctionMap};
use geo_types::Point;
use shylock_data::types::{Asset, Auction, LotAuctionKind, Management};
use shylock_data::AuctionState;
use std::collections::HashMap;

const DEFAULT_COUNTRY: &str = "Spain";

/// Constant for parsing lot number @TODO make pub(crate) in future
pub const AUCTION_LOT_NUMBER_STR: &str = "idLote=";

fn get_auctions_links(
    url_fetcher: &dyn HttpClient,
) -> Result<Vec<(String, AuctionState)>, Box<dyn std::error::Error>> {
    let main_page = url_fetcher.get_url(&MAIN_ONGOING_AUCTIONS_BOE_URL)?;
    let mut result = parse_result_page(&main_page);

    for page_url in parse_extra_pages(&main_page) {
        let extra_page = url_fetcher.get_url(&page_url)?;
        result.append(&mut parse_result_page(&extra_page));
    }

    Ok(result)
}

fn update_asset_coordinates(asset: &mut Asset, geosolver: &GeoSolver) {
    if let Asset::Property(property) = asset {
        property.coordinates = match geosolver.resolve(
            &property.address,
            &property.city,
            property.province.name(),
            DEFAULT_COUNTRY,
            &property.postal_code,
        ) {
            Ok(coordinates) => coordinates,
            Err(error) => {
                log::warn!("Unable to retrieve coordinates: {}", error);
                Some(Point::new(0.0, 0.0))
            }
        };
    }
}

fn process_auction_link(
    url_fetcher: &dyn HttpClient,
    link: &(String, AuctionState),
) -> Result<(Auction, Vec<Asset>), Box<dyn std::error::Error>> {
    let mut assets = Vec::new();
    let geosolver = GeoSolver::new();

    let auction_page = url_fetcher.get_url(&link.0)?;

    let (mgm_link, asset_link) = parse_main_auction_links(&auction_page)?;
    let management_page = url_fetcher.get_url(&mgm_link)?;
    let management = Management::new(&parse_management_auction_page(&management_page)?);
    log::info!("Created management: {}", management.code);

    let auction = Auction::new(&parse_main_auction_page(&auction_page)?, management, link.1);
    log::info!("Created auction: {}", auction.id);

    let asset_page = url_fetcher.get_url(&asset_link)?;
    match auction.lot_kind {
        LotAuctionKind::NotApplicable => {
            let mut asset = Asset::new(&auction.id, &parse_asset_auction_page(&asset_page)?);
            update_asset_coordinates(&mut asset, &geosolver);

            assets.push(asset);
        }
        LotAuctionKind::Joined | LotAuctionKind::Splitted => {
            let lot_links = parse_lot_auction_page_links(&asset_page)?;
            for lot_link in lot_links.iter() {
                let lot_page = url_fetcher.get_url(lot_link)?;

                let lot_id_begin =
                    lot_link.find(AUCTION_LOT_NUMBER_STR).unwrap() + AUCTION_LOT_NUMBER_STR.len();
                let lot_id_end = lot_link[lot_id_begin..].find('&').unwrap() + lot_id_begin;
                let lot_id = &lot_link[lot_id_begin..lot_id_end];

                let mut asset = Asset::new(
                    &auction.id,
                    &parse_lot_auction_page(&lot_page, lot_id.parse::<usize>().unwrap())?,
                );
                update_asset_coordinates(&mut asset, &geosolver);
                assets.push(asset);
            }
        }
    }

    Ok((auction, assets))
}

/// Scrape subastas.boe.es to get all assets in auctions.
pub fn scrape() -> Result<(AuctionMap, Vec<Asset>), Box<dyn std::error::Error>> {
    let url_fetcher = BlockingUrlFetcher::new();
    let mut total_assets = Vec::new();
    let mut total_auctions = HashMap::new();
    let auction_links = get_auctions_links(&url_fetcher)?;
    let number_auctions = auction_links.len();

    log::info!("Total auctions to process: {}", number_auctions);
    let mut auction_ok = 0;
    let mut auction_err = 0;

    for auction_link in auction_links {
        match process_auction_link(&url_fetcher, &auction_link) {
            Ok((auction, mut auction_assets)) => {
                auction_ok += 1;
                total_auctions.insert(auction.id.clone(), auction);
                total_assets.append(&mut auction_assets)
            }
            Err(err) => {
                auction_err += 1;
                log::warn!("Unable to process: {}", err)
            }
        }
        log::info!(
            "Auctions processed: {}/{}, Auctions errors: {}",
            auction_ok + auction_err,
            number_auctions,
            auction_err
        );
    }

    Ok((total_auctions, total_assets))
}
