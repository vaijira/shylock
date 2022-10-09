use geo_types::Point;
use rust_decimal::Decimal;
use shylock_data::{
    Asset, Auction, AuctionState, BidInfo, Management, Other, Property, Vehicle, DEFAULT_DECIMALS,
};
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow, SqliteSynchronous,
    },
    Pool, Row, Sqlite,
};
use std::fmt::Write;
use std::{str::FromStr, time::Duration};

use crate::util::normalize;

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

    /// Returns all auctions with a determine `state`.
    pub async fn get_auction_ids_with_states(
        &self,
        states: &[AuctionState],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut select_query = "SELECT id FROM auctions WHERE ".to_owned();

        for (i, state) in states.iter().enumerate() {
            if i > 0 {
                write!(select_query, "or auction_state = '{}' ", state)?;
            } else {
                write!(select_query, "auction_state = '{}' ", state)?;
            }
        }

        Ok(sqlx::query(&select_query)
            .map(|row: SqliteRow| row.get(0))
            .fetch_all(&self.pool)
            .await?)
    }

    /// Update `auction_id` auction with the new `state`.
    pub async fn update_auction_state(
        &self,
        auction_id: &str,
        state: AuctionState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"UPDATE auctions
        SET auction_state = ?
        WHERE id = ?"#,
        )
        .bind(state)
        .bind(auction_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update `property` coordinates in db.
    pub async fn update_asset_coordinate(
        &self,
        property: &Property,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let point = property.coordinates.unwrap().x_y();
        sqlx::query(
            r#"UPDATE properties
            SET coordinates = ?
            WHERE auction_id = ? and
            catastro_reference = ?"#,
        )
        .bind(&format!("{} {}", point.0, point.1))
        .bind(&property.auction_id)
        .bind(&property.catastro_reference)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get management by `id`.
    pub async fn get_management_by_id(
        &self,
        id: &str,
    ) -> Result<Management, Box<dyn std::error::Error>> {
        match sqlx::query(r#"SELECT * FROM managements WHERE code = ?"#)
            .bind(&id)
            .map(|row: SqliteRow| Management {
                code: row.get("code"),
                description: row.get("description"),
                address: row.get("address"),
                telephone: row.get("telephone"),
                fax: row.get("fax"),
                email: row.get("email"),
            })
            .fetch_one(&self.pool)
            .await
        {
            Ok(management) => Ok(management),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Returns all auctions with determine `states`.
    pub async fn get_auctions_with_states(
        &self,
        states: &[AuctionState],
    ) -> Result<Vec<Auction>, Box<dyn std::error::Error>> {
        let mut select_query =
            "SELECT a.*, m.* FROM auctions a JOIN managements m ON a.management = m.code WHERE "
                .to_owned();

        for (i, state) in states.iter().enumerate() {
            if i > 0 {
                write!(select_query, "or a.auction_state = '{}' ", state)?;
            } else {
                write!(select_query, "a.auction_state = '{}' ", state)?;
            }
        }

        Ok(sqlx::query(&select_query)
            .map(|row: SqliteRow| {
                let claim_quantity: i64 = row.get_unchecked("claim_quantity");
                let bidinfo: String = row.get("bidinfo");
                let management = Management {
                    code: row.get("code"),
                    description: row.get("description"),
                    address: row.get("address"),
                    telephone: row.get("telephone"),
                    fax: row.get("fax"),
                    email: row.get("email"),
                };

                Auction {
                    id: row.get("id"),
                    auction_state: row.get("auction_state"),
                    kind: row.get("kind"),
                    claim_quantity: Decimal::new(claim_quantity, DEFAULT_DECIMALS),
                    lots: row.get("lots"),
                    lot_kind: row.get("lot_kind"),
                    management,
                    bidinfo: BidInfo::from_str(&bidinfo).unwrap(),
                    start_date: row.get("start_date"),
                    end_date: row.get("end_date"),
                    notice: row.get("notice"),
                }
            })
            .fetch_all(&self.pool)
            .await?)
    }

    /// Returns all properties with determine auction `states`.
    pub async fn get_properties_with_auction_states(
        &self,
        states: &[AuctionState],
    ) -> Result<Vec<Property>, Box<dyn std::error::Error>> {
        let mut select_query =
            "SELECT p.* FROM properties p JOIN auctions a ON p.auction_id = a.id WHERE ".to_owned();

        for (i, state) in states.iter().enumerate() {
            if i > 0 {
                write!(select_query, "or a.auction_state = '{}' ", state)?;
            } else {
                write!(select_query, "a.auction_state = '{}' ", state)?;
            }
        }

        Ok(sqlx::query(&select_query)
            .map(|row: SqliteRow| {
                let charges: i64 = row.get_unchecked("charges");
                let bidinfo: String = row.get("bidinfo");
                let points_str: Option<String> = row.get("coordinates");
                let coordinates = if let Some(points) = points_str {
                    let points = points.split(' ').collect::<Vec<&str>>();
                    Some(Point::new(
                        points[0].parse::<f64>().unwrap(),
                        points[1].parse::<f64>().unwrap(),
                    ))
                } else {
                    None
                };

                Property {
                    address: row.get("address"),
                    auction_id: row.get("auction_id"),
                    bidinfo: match BidInfo::from_str(&bidinfo) {
                        Ok(v) => Some(v),
                        _ => None,
                    },
                    catastro_reference: row.get("catastro_reference"),
                    category: row.get("category"),
                    charges: Decimal::new(charges, DEFAULT_DECIMALS),
                    city: normalize(row.get("city")),
                    coordinates,
                    description: row.get("description"),
                    owner_status: row.get("owner_status"),
                    postal_code: row.get("postal_code"),
                    primary_residence: row.get("primary_residence"),
                    province: row.get("province"),
                    register_inscription: row.get("register_inscription"),
                    visitable: row.get("visitable"),
                }
            })
            .fetch_all(&self.pool)
            .await?)
    }

    /// Returns all vehicles with determine auction `states`.
    pub async fn get_vehicles_with_auction_states(
        &self,
        states: &[AuctionState],
    ) -> Result<Vec<Vehicle>, Box<dyn std::error::Error>> {
        let mut select_query =
            "SELECT v.* FROM vehicles v JOIN auctions a ON v.auction_id = a.id WHERE ".to_owned();

        for (i, state) in states.iter().enumerate() {
            if i > 0 {
                write!(select_query, "or a.auction_state = '{}' ", state)?;
            } else {
                write!(select_query, "a.auction_state = '{}' ", state)?;
            }
        }

        Ok(sqlx::query(&select_query)
            .map(|row: SqliteRow| {
                let charges: i64 = row.get_unchecked("charges");
                let bidinfo: String = row.get("bidinfo");

                Vehicle {
                    auction_id: row.get("auction_id"),
                    bidinfo: match BidInfo::from_str(&bidinfo) {
                        Ok(v) => Some(v),
                        _ => None,
                    },
                    brand: row.get("brand"),
                    category: row.get("category"),
                    charges: Decimal::new(charges, DEFAULT_DECIMALS),
                    description: row.get("description"),
                    frame_number: row.get("frame_number"),
                    licensed_date: row.get("licensed_date"),
                    license_plate: row.get("license_plate"),
                    localization: row.get("localization"),
                    model: row.get("model"),
                    visitable: row.get("visitable"),
                }
            })
            .fetch_all(&self.pool)
            .await?)
    }

    /// Returns all other asstes with determine auction `states`.
    pub async fn get_other_assets_with_auction_states(
        &self,
        states: &[AuctionState],
    ) -> Result<Vec<Other>, Box<dyn std::error::Error>> {
        let mut select_query =
            "SELECT o.* FROM others o JOIN auctions a ON o.auction_id = a.id WHERE ".to_owned();

        for (i, state) in states.iter().enumerate() {
            if i > 0 {
                write!(select_query, "or a.auction_state = '{}' ", state)?;
            } else {
                write!(select_query, "a.auction_state = '{}' ", state)?;
            }
        }

        Ok(sqlx::query(&select_query)
            .map(|row: SqliteRow| {
                let charges: i64 = row.get_unchecked("charges");
                let bidinfo: String = row.get("bidinfo");

                Other {
                    additional_information: row.get("additional_information"),
                    auction_id: row.get("auction_id"),
                    bidinfo: match BidInfo::from_str(&bidinfo) {
                        Ok(v) => Some(v),
                        _ => None,
                    },
                    category: row.get("category"),
                    charges: Decimal::new(charges, DEFAULT_DECIMALS),
                    description: row.get("description"),
                    judicial_title: row.get("judicial_title"),
                    visitable: row.get("visitable"),
                }
            })
            .fetch_all(&self.pool)
            .await?)
    }
}
