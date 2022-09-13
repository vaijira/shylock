use clap::{arg, Command};
use env_logger::Env;
use futures::*;
use shylock_data::types::{Asset, Auction, LotAuctionKind, Management, Other, Property, Vehicle};
use shylock_parser::{
    http::{UrlFetcher, MAIN_ALL_AUCTIONS_BOE_URL},
    scraper::AUCTION_LOT_NUMBER_STR,
    AuctionState,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};

use std::str::FromStr;
use std::time::Duration;

const DEFAULT_DB_PATH: &str = "./db/shylock.db";
const DEFAULT_POOL_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_CONNECTIONS: u32 = 5;
const AUCTION_ID_LINK_STR: &str = "?idSub=";

async fn insert_other_asset(sqlite_pool: &Pool<Sqlite>, auction: &Auction, other: &Other) {
    sqlx::query(
        r#"
    INSERT INTO others(
        additional_information, auction_id,
        bidinfo, category, charges,
        description, judicial_title,
        visitable
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&other.additional_information)
    .bind(&auction.id)
    .bind(other.bidinfo.as_ref().map(|bidinfo| bidinfo.to_string()))
    .bind(&other.category)
    .bind(&other.charges.to_string())
    .bind(&other.description)
    .bind(&other.judicial_title)
    .bind(&other.visitable)
    .execute(sqlite_pool)
    .await
    .expect("Inserting asset other in db");
}

async fn insert_property_asset(sqlite_pool: &Pool<Sqlite>, auction: &Auction, property: &Property) {
    sqlx::query(
        r#"
    INSERT INTO properties(
        address, auction_id, bidinfo,
        catastro_reference, category,
        charges, city, description,
        owner_status, postal_code,
        primary_residence, province,
        register_inscription, visitable
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&property.address)
    .bind(&auction.id)
    .bind(property.bidinfo.as_ref().map(|bidinfo| bidinfo.to_string()))
    .bind(&property.catastro_reference)
    .bind(&property.category)
    .bind(&property.charges.to_string())
    .bind(&property.city)
    .bind(&property.description)
    .bind(&property.owner_status)
    .bind(&property.postal_code)
    .bind(&property.primary_residence)
    .bind(&property.province)
    .bind(&property.register_inscription)
    .bind(&property.visitable)
    .execute(sqlite_pool)
    .await
    .expect("Inserting asset property in db");
}

async fn insert_vehicle_asset(sqlite_pool: &Pool<Sqlite>, auction: &Auction, vehicle: &Vehicle) {
    sqlx::query(
        r#"
    INSERT INTO vehicles(
        auction_id, bidinfo, brand,
        category, charges, description,
        frame_number, licensed_date,
        license_plate, localization,
        model, visitable
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&auction.id)
    .bind(vehicle.bidinfo.as_ref().map(|bidinfo| bidinfo.to_string()))
    .bind(&vehicle.brand)
    .bind(&vehicle.category)
    .bind(&vehicle.charges.to_string())
    .bind(&vehicle.description)
    .bind(&vehicle.frame_number)
    .bind(&vehicle.licensed_date)
    .bind(&vehicle.license_plate)
    .bind(&vehicle.localization)
    .bind(&vehicle.model)
    .bind(&vehicle.visitable)
    .execute(sqlite_pool)
    .await
    .expect("Inserting asset vehicle in db");
}

async fn insert_assets(sqlite_pool: &Pool<Sqlite>, auction: &Auction, assets: &Vec<Asset>) {
    for asset in assets {
        match asset {
            Asset::Other(other) => insert_other_asset(sqlite_pool, auction, other).await,

            Asset::Property(property) => {
                insert_property_asset(sqlite_pool, auction, property).await
            }
            Asset::Vehicle(vehicle) => insert_vehicle_asset(sqlite_pool, auction, vehicle).await,
        }
    }
}

async fn insert_auction(sqlite_pool: &Pool<Sqlite>, auction: &Auction) {
    sqlx::query(
        r#"INSERT INTO auctions(
        id, auction_state, kind, claim_quantity,
        lots, lot_kind, management, bidinfo,
        start_date, end_date, notice) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&auction.id)
    .bind(&auction.auction_state)
    .bind(&auction.kind)
    .bind(&auction.claim_quantity.to_string())
    .bind(&auction.lots)
    .bind(&auction.lot_kind)
    .bind(&auction.management.code)
    .bind(&auction.bidinfo.to_string())
    .bind(&auction.start_date)
    .bind(&auction.end_date)
    .bind(&auction.notice)
    .execute(sqlite_pool)
    .await
    .expect("Inserting auction in db");
}

async fn insert_management(sqlite_pool: &Pool<Sqlite>, management: &Management) {
    sqlx::query(
        r#"INSERT INTO managements(
        code, description, address, telephone, fax, email)
        VALUES (?, ?, ?, ?, ?, ?)
        ON CONFLICT(code)
            DO UPDATE SET
            description = excluded.description,
            address = excluded.address,
            telephone = excluded.telephone,
            fax = excluded.fax,
            email = excluded.email
        "#,
    )
    .bind(&management.code)
    .bind(&management.description)
    .bind(&management.address)
    .bind(&management.telephone)
    .bind(&management.fax)
    .bind(&management.email)
    .execute(sqlite_pool)
    .await
    .expect("Inserting management in db");
}

async fn auction_exists(
    sqlite_pool: &Pool<Sqlite>,
    id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    match sqlx::query(r#"SELECT id FROM auctions WHERE id = ?"#)
        .bind(&id)
        .fetch_optional(sqlite_pool)
        .await
    {
        Ok(Some(_)) => Ok(true),
        Ok(None) | Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(err) => Err(Box::new(err)),
    }
}

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
    sqlite_pool: &Pool<Sqlite>,
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

        if let Ok(true) = auction_exists(sqlite_pool, auction_id).await {
            log::info!("Auction ->{}<- previously processed", auction_id);
            auction_already_process += 1;
            continue;
        }
        match process_auction_link(http_client, &auction_link).await {
            Ok((auction, auction_assets)) => {
                let tx = sqlite_pool.begin().await?;

                insert_management(sqlite_pool, &auction.management).await;

                insert_auction(sqlite_pool, &auction).await;

                insert_assets(sqlite_pool, &auction, &auction_assets).await;

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

async fn scrape(sqlite_pool: &Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = &UrlFetcher::new();
    let main_page = http_client.get_url(&MAIN_ALL_AUCTIONS_BOE_URL).await?;
    let mut pages_url = shylock_parser::parser::parse_extra_pages(&main_page);

    pages_url.insert(0, MAIN_ALL_AUCTIONS_BOE_URL.to_string());
    log::info!("Total BOE pages to process: {}", pages_url.len());

    let stream = stream::iter(pages_url.iter().enumerate());

    stream
        .for_each_concurrent(6, |page| async move {
            if let Ok((ok, err, already_proccessed)) =
                page_scraper(http_client, sqlite_pool, page.1).await
            {
                log::info!(
                    "Page {} ended succesfully {}/{}/{}",
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
    let db_url = format!("sqlite://{}", db_path);

    let connection_options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(DEFAULT_POOL_TIMEOUT);

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(DEFAULT_MAX_CONNECTIONS)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./sql").run(&sqlite_pool).await?;

    let _ = scrape(&sqlite_pool).await;

    Ok(())
}
