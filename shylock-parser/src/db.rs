use shylock_data::{Asset, Auction, Management, Other, Property, Vehicle};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};
use std::{str::FromStr, time::Duration};

/// Default path for db file.
pub const DEFAULT_DB_PATH: &str = "./db/shylock.db";
const DEFAULT_POOL_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_CONNECTIONS: u32 = 5;

/// Database client.
#[derive(Debug)]
pub struct DbClient {
    /// database pool
    pub pool: Pool<Sqlite>,
}

impl DbClient {
    /// Create new client with default options
    pub async fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
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

        Ok(DbClient { pool: sqlite_pool })
    }

    /// Create new database client from a pool.
    pub fn from_pool(client_pool: Pool<Sqlite>) -> Self {
        DbClient { pool: client_pool }
    }

    /// Execute migration sql scripts.
    pub async fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::migrate!("./sql").run(&self.pool).await?;
        Ok(())
    }

    async fn insert_other_asset(&self, auction: &Auction, other: &Other) {
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
        .execute(&self.pool)
        .await
        .expect("Inserting asset other in db");
    }

    async fn insert_property_asset(&self, auction: &Auction, property: &Property) {
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
        .execute(&self.pool)
        .await
        .expect("Inserting asset property in db");
    }

    async fn insert_vehicle_asset(&self, auction: &Auction, vehicle: &Vehicle) {
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
        .execute(&self.pool)
        .await
        .expect("Inserting asset vehicle in db");
    }

    /// Insert `auction` `assets` (Property, vehicle and other) in db.
    pub async fn insert_assets(&self, auction: &Auction, assets: &Vec<Asset>) {
        for asset in assets {
            match asset {
                Asset::Other(other) => self.insert_other_asset(auction, other).await,

                Asset::Property(property) => self.insert_property_asset(auction, property).await,
                Asset::Vehicle(vehicle) => self.insert_vehicle_asset(auction, vehicle).await,
            }
        }
    }

    /// Insert `auction` in db.
    pub async fn insert_auction(&self, auction: &Auction) {
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
        .execute(&self.pool)
        .await
        .expect("Inserting auction in db");
    }

    /// Insert `management` information in db.
    pub async fn insert_management(&self, management: &Management) {
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
        .execute(&self.pool)
        .await
        .expect("Inserting management in db");
    }

    /// Check if a auction with `id` is already in db.
    pub async fn auction_exists(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match sqlx::query(r#"SELECT id FROM auctions WHERE id = ?"#)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(Some(_)) => Ok(true),
            Ok(None) | Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(err) => Err(Box::new(err)),
        }
    }
}
