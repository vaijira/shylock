use crate::categories::PropertyCategory;
use crate::concepts::BoeConcept;
use crate::provinces::Province;

use chrono::NaiveDate;
use geo_types::Point;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) const DEFAULT_DECIMALS: u32 = 2;
const NOT_APPLICABLE: &str = "NA";

fn get_clean_text(data: &HashMap<BoeConcept, String>, field: &BoeConcept) -> String {
    if let Some(field_str) = data.get(field) {
        field_str
            .replace(",", ", ")
            .replace(".", ". ")
            .replace("  ", " ")
            .trim()
            .to_string()
    } else {
        String::from(NOT_APPLICABLE)
    }
}

fn get_date(data: &HashMap<BoeConcept, String>, field: &BoeConcept) -> NaiveDate {
    if let Some(date_str) = data.get(field) {
        let space_offset = date_str.find(' ').unwrap_or_else(|| date_str.len());
        NaiveDate::parse_from_str(&date_str[..space_offset], "%d-%m-%Y").unwrap()
    } else {
        NaiveDate::parse_from_str("01-01-2000", "%d-%m-%Y").unwrap()
    }
}

fn get_vehicle_date(data: &HashMap<BoeConcept, String>, field: &BoeConcept) -> NaiveDate {
    if let Some(date_str) = data.get(field) {
        let date_str = date_str.replace("/", "-");
        match NaiveDate::parse_from_str(&date_str[..], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => match NaiveDate::parse_from_str(&date_str[..], "%d-%m-%Y") {
                Ok(date) => date,
                Err(error) => {
                    log::warn!(
                        "Unable to parse date {}: {}",
                        &date_str[..],
                        error.to_string()
                    );
                    NaiveDate::parse_from_str("01-01-2000", "%d-%m-%Y").unwrap()
                }
            },
        }
    } else {
        NaiveDate::parse_from_str("01-01-2000", "%d-%m-%Y").unwrap()
    }
}

fn get_decimal(data: &HashMap<BoeConcept, String>, field: &BoeConcept) -> Decimal {
    if let Some(decimal_str) = data.get(field) {
        let space_offset = decimal_str.find(' ').unwrap_or_else(|| decimal_str.len());
        let result = str::replace(&decimal_str[..space_offset], ".", "");
        let result = str::replace(&result[..], ",", "");
        Decimal::new(result.parse::<i64>().unwrap_or(0), DEFAULT_DECIMALS)
    } else {
        Decimal::new(0, DEFAULT_DECIMALS)
    }
}

fn get_auction_kind(data: &HashMap<BoeConcept, String>) -> AuctionKind {
    let mut result: AuctionKind = AuctionKind::Unkown;
    if let Some(auction_kind) = data.get(&BoeConcept::AuctionKind) {
        result = match &auction_kind[..] {
            "AGENCIA TRIBUTARIA" => AuctionKind::TaxAgency,
            "RECAUDACIÓN TRIBUTARIA" => AuctionKind::TaxCollection,
            "NOTARIAL VOLUNTARIA" => AuctionKind::NotaryVoluntary,
            "JUDICIAL VOLUNTARIA" => AuctionKind::JudicialVoluntary,
            "JUDICIAL EN VIA DE APREMIO" => AuctionKind::JudicialUnderPressure,
            "JUDICIAL CONCURSAL" => AuctionKind::Bankruptcy,
            "NOTARIAL EN VENTA EXTRAJUDICIAL" => AuctionKind::NotaryExtraJudicial,
            _ => AuctionKind::Unkown,
        };
    }

    result
}

fn get_lot_auction_kind(data: &HashMap<BoeConcept, String>) -> LotAuctionKind {
    let mut result: LotAuctionKind = LotAuctionKind::NotApplicable;

    if let Some(auction_kind) = data.get(&BoeConcept::LotAuctionKind) {
        result = match &auction_kind[..] {
            "CONJUNTA PARA TODOS LOS LOTES" => LotAuctionKind::Joined,
            "SEPARADA PARA CADA LOTE" => LotAuctionKind::Splitted,
            _ => LotAuctionKind::NotApplicable,
        };
    }

    result
}

/// Manager information to contact for information about the auction.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Management {
    /// Management code
    pub code: String,
    /// Description
    pub description: String,
    /// Contact address
    pub address: String,
    /// Contact telephone
    pub telephone: String,
    /// Conctact fax
    pub fax: String,
    /// Contact email
    pub email: String,
}

impl Management {
    /// Create a new Management
    pub fn new(data: &HashMap<BoeConcept, String>) -> Management {
        Management {
            code: data
                .get(&BoeConcept::Code)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            description: get_clean_text(data, &BoeConcept::Description),
            address: data
                .get(&BoeConcept::Address)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            telephone: data
                .get(&BoeConcept::Telephone)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            fax: data
                .get(&BoeConcept::Fax)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            email: data
                .get(&BoeConcept::Email)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
        }
    }
}

/// Bid information struct
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BidInfo {
    /// Valuation of the assets.
    pub appraisal: Decimal,
    /// Steps for each bid
    pub bid_step: Decimal,
    /// Claim quantity
    pub claim_quantity: Decimal,
    /// Deposit if someone wants to participate in the auction
    pub deposit: Decimal,
    /// Minimum bid for the auction.
    pub minimum_bid: Decimal,
    /// Auction value.
    pub value: Decimal,
}

impl BidInfo {
    /// Create a new Auction
    pub fn new(data: &HashMap<BoeConcept, String>) -> BidInfo {
        BidInfo {
            appraisal: get_decimal(data, &BoeConcept::Appraisal),
            bid_step: get_decimal(data, &BoeConcept::BidStep),
            claim_quantity: get_decimal(data, &BoeConcept::ClaimQuantity),
            deposit: get_decimal(data, &BoeConcept::DepositAmount),
            minimum_bid: get_decimal(data, &BoeConcept::MinimumBid),
            value: get_decimal(data, &BoeConcept::AuctionValue),
        }
    }
}

/// All posible kind of auctions.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AuctionKind {
    /// Tax agency auction
    TaxAgency, // AGENCIA TRIBUTARIA
    /// Tax collection auction
    TaxCollection, // RECAUDACIÓN TRIBUTARIA
    /// Notary voluntary
    NotaryVoluntary, // NOTARIAL VOLUNTARIA
    /// Judicial voluntary
    JudicialVoluntary, // JUDICIAL VOLUNTARIA
    /// Judicial under pressure
    JudicialUnderPressure, // JUDICIAL EN VIA DE APREMIO
    /// Bankruptcy
    Bankruptcy, // JUDICIAL CONCURSAL
    /// Notary extra judicial sell
    NotaryExtraJudicial, // NOTARIAL EN VENTA EXTRAJUDICIAL
    /// Unkown
    Unkown,
}

/// Type of auction kind when it contains lots
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LotAuctionKind {
    /// Auction without lots
    NotApplicable,
    /// Joined lots in auction
    Joined,
    /// Splitted lots in auction
    Splitted,
}

/// Auction struct
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Auction {
    /// Auction unique identifier.
    pub id: String,
    /// Type of auction classified by entity.
    pub kind: AuctionKind,
    /// Quantity that is claimed by creditors.
    pub claim_quantity: Decimal,
    /// Number of lots.
    pub lots: u32,
    /// Kind of lots (splitted or joined).
    pub lot_kind: LotAuctionKind,
    /// Auction management.
    pub management: Management,
    /// BidInfo.
    pub bidinfo: BidInfo,
    /// When the auction starts.
    pub start_date: NaiveDate,
    /// End date for auction.
    pub end_date: NaiveDate,
    /// Notice in official bulletin
    pub notice: String,
}

impl Auction {
    /// Create a new Auction
    pub fn new(data: &HashMap<BoeConcept, String>, management: Management) -> Auction {
        let lots: u32 = data
            .get(&BoeConcept::Lots)
            .unwrap_or(&"0".to_owned())
            .parse::<u32>()
            .unwrap_or(0);

        Auction {
            id: data
                .get(&BoeConcept::Identifier)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            kind: get_auction_kind(data),
            claim_quantity: get_decimal(data, &BoeConcept::ClaimQuantity),
            lots,
            lot_kind: get_lot_auction_kind(data),
            management,
            bidinfo: BidInfo::new(data),
            start_date: get_date(data, &BoeConcept::StartDate),
            end_date: get_date(data, &BoeConcept::EndDate),
            notice: data
                .get(&BoeConcept::Notice)
                .unwrap_or(&String::from("BOE"))
                .to_string(),
        }
    }
}

/// Property can be any real state property: apartment, garage lot, industrial ...
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Property {
    /// Address location.
    pub address: String,
    /// Unique identifier is linked to.
    pub auction_id: String,
    /// Bid info
    pub bidinfo: Option<BidInfo>,
    /// Catastro reference.
    pub catastro_reference: String,
    /// category, usually: industrial, garage or apartment.
    pub category: PropertyCategory,
    /// If the property has any previous charges.
    pub charges: Decimal,
    /// City the asset is in.
    pub city: String,
    /// Location coordinates.
    pub coordinates: Option<Point<f64>>,
    /// Description.
    pub description: String,
    /// Owner staus.
    pub owner_status: String,
    /// Postal code.
    pub postal_code: String,
    /// Indicates if it is primary residence.
    pub primary_residence: String,
    /// Province.
    pub province: Province,
    /// Register inscription.
    pub register_inscription: String,
    /// If someone can visit the property or not.
    pub visitable: String,
}

impl Eq for Property {}

impl Property {
    /// Create a new property asset.
    pub fn new(
        auction: &str,
        category: PropertyCategory,
        data: &HashMap<BoeConcept, String>,
    ) -> Property {
        let city = data
            .get(&BoeConcept::City)
            .unwrap_or(&String::from(NOT_APPLICABLE))
            .to_string();
        let province = data
            .get(&BoeConcept::Province)
            .unwrap_or(&String::from("Unknown"))
            .parse::<Province>()
            .unwrap();
        let postal_code = data
            .get(&BoeConcept::PostalCode)
            .unwrap_or(&String::from(NOT_APPLICABLE))
            .to_string();
        let bidinfo = if data.get(&BoeConcept::AuctionValue).is_some() {
            Some(BidInfo::new(data))
        } else {
            None
        };
        Property {
            address: data
                .get(&BoeConcept::Address)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            auction_id: auction.to_string(),
            bidinfo,
            catastro_reference: data
                .get(&BoeConcept::CatastroReference)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            category,
            charges: get_decimal(data, &BoeConcept::Charges),
            city,
            coordinates: None,
            description: get_clean_text(data, &BoeConcept::Description),
            owner_status: data
                .get(&BoeConcept::OwnerStatus)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            postal_code,
            primary_residence: data
                .get(&BoeConcept::PrimaryResidence)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            province,
            register_inscription: data
                .get(&BoeConcept::RegisterInscription)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            visitable: data
                .get(&BoeConcept::Visitable)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
        }
    }
}

/// Any kind of vehicle
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Vehicle {
    /// Auction identifier is linked to.
    pub auction_id: String,
    /// Bid info
    pub bidinfo: Option<BidInfo>,
    /// Vehicle brand.
    pub brand: String,
    /// If vehicle has previous charges.
    pub charges: Decimal,
    /// Description.
    pub description: String,
    /// Frame number.
    pub frame_number: String, // Número de bastidor
    /// Licensed date.
    pub licensed_date: NaiveDate,
    /// License plate number.
    pub license_plate: String,
    /// Localization.
    pub localization: String,
    /// Model.
    pub model: String,
    /// Subcategory, usually: car, motorbike or industrial.
    pub subcategory: String,
    /// Indicates if someone can inspect the vehicle.
    pub visitable: String,
}

impl Vehicle {
    /// Create a new vehicle asset.
    pub fn new(auction: &str, subcategory: &str, data: &HashMap<BoeConcept, String>) -> Vehicle {
        let bidinfo = if data.get(&BoeConcept::AuctionValue).is_some() {
            Some(BidInfo::new(data))
        } else {
            None
        };
        Vehicle {
            auction_id: auction.to_string(),
            bidinfo,
            brand: data
                .get(&BoeConcept::Brand)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            charges: get_decimal(data, &BoeConcept::Charges),
            description: get_clean_text(data, &BoeConcept::Description),
            frame_number: data
                .get(&BoeConcept::FrameNumber)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            licensed_date: get_vehicle_date(data, &BoeConcept::LicensedDate),
            license_plate: data
                .get(&BoeConcept::LicensePlate)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            localization: data
                .get(&BoeConcept::Localization)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            model: data
                .get(&BoeConcept::Model)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            subcategory: subcategory.to_string(),
            visitable: data
                .get(&BoeConcept::Visitable)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
        }
    }
}

/// Any asset that is not a vehicle or a property.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Other {
    /// Any asset additional information.
    pub additional_information: String,
    /// Auction is linked to.
    pub auction_id: String,
    /// Bid info
    pub bidinfo: Option<BidInfo>,
    /// If the asset has any previous charges.
    pub charges: Decimal,
    /// Description.
    pub description: String,
    /// Type of judicial title if applies.
    pub judicial_title: String,
    /// Subcategory.
    pub subcategory: String,
    /// If someone can visit the asset if applies.
    pub visitable: String,
}

impl Other {
    /// Create an asset that is not a vehicle or real state property.
    pub fn new(auction: &str, subcategory: &str, data: &HashMap<BoeConcept, String>) -> Other {
        let bidinfo = if data.get(&BoeConcept::AuctionValue).is_some() {
            Some(BidInfo::new(data))
        } else {
            None
        };
        Other {
            additional_information: data
                .get(&BoeConcept::AdditionalInformation)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            auction_id: auction.to_string(),
            bidinfo,
            charges: get_decimal(data, &BoeConcept::Charges),
            description: get_clean_text(data, &BoeConcept::Description),
            judicial_title: data
                .get(&BoeConcept::JudicialTitle)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            subcategory: subcategory.to_string(),
            visitable: data
                .get(&BoeConcept::Visitable)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
        }
    }
}

/// Type of assets
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Asset {
    /// All except vehicle or property
    Other(Other),
    /// Any kind of real state property
    Property(Property),
    /// Any kind of vehicle
    Vehicle(Vehicle),
}

impl Asset {
    fn parse_header(header: &str) -> (String, String) {
        let begin_cat = header.find('-').unwrap() + 2;
        if let Some(separator) = header.find('(') {
            let end_cat = separator - 1;
            let begin_subcat = separator + 1;
            let end_subcat = header.find(')').unwrap();

            (
                header[begin_cat..end_cat].to_owned(),
                header[begin_subcat..end_subcat].to_owned(),
            )
        } else {
            (header[begin_cat..].to_owned(), "".to_owned())
        }
    }

    /// Create a new Asset
    pub fn new(auction: &str, data: &HashMap<BoeConcept, String>) -> Asset {
        let header = data.get(&BoeConcept::Header).unwrap().to_string();
        let (category, subcategory) = Asset::parse_header(&header);
        match &category[..] {
            "INMUEBLE" => {
                let property_category = match subcategory.parse::<PropertyCategory>() {
                    Ok(cat) => cat,
                    Err(err) => {
                        log::error!("Unable to parse property category: {}", err);
                        PropertyCategory::Apartment
                    }
                };
                Asset::Property(Property::new(auction, property_category, data))
            }
            "VEHÍCULO" => Asset::Vehicle(Vehicle::new(auction, &subcategory, data)),
            _ => Asset::Other(Other::new(auction, &subcategory, data)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_parse_header_test() {
        let (cat, subcat) = Asset::parse_header("BIEN 0 - INMUEBLE (VIVIENDA)");
        assert_eq!("INMUEBLE", cat);
        assert_eq!("VIVIENDA", subcat);

        let (cat, subcat) = Asset::parse_header("BIEN 1 - VEHÍCULO (INDUSTRIALES)");
        assert_eq!("VEHÍCULO", cat);
        assert_eq!("INDUSTRIALES", subcat);

        let (cat, subcat) = Asset::parse_header("BIEN 1 - BIEN MUEBLE (OTROS BIENES Y DERECHOS)");
        assert_eq!("BIEN MUEBLE", cat);
        assert_eq!("OTROS BIENES Y DERECHOS", subcat);

        let (cat, subcat) = Asset::parse_header("BIEN 0 - INMUEBLE");
        assert_eq!("INMUEBLE", cat);
        assert_eq!("", subcat);
    }

    #[test]
    fn get_date_test() {
        let data: HashMap<BoeConcept, String> = [
            (
                BoeConcept::StartDate,
                String::from("14-07-2020 18:00:00 CET  (ISO: 2020-07-14T18:00:00+02:00)"),
            ),
            (
                BoeConcept::EndDate,
                String::from("03-08-2020 18:00:00 CET  (ISO: 2020-08-03T18:00:00+02:00)"),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(
            NaiveDate::from_ymd(2020, 7, 14),
            get_date(&data, &BoeConcept::StartDate)
        );
        assert_eq!(
            NaiveDate::from_ymd(2020, 8, 3),
            get_date(&data, &BoeConcept::EndDate)
        );
    }

    #[test]
    fn get_decimal_test() {
        let data: HashMap<BoeConcept, String> = [
            (BoeConcept::ClaimQuantity, String::from("81.971,57 €")),
            (BoeConcept::Appraisal, String::from("75.127,00 €")),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(
            Decimal::new(8197157, DEFAULT_DECIMALS),
            get_decimal(&data, &BoeConcept::ClaimQuantity)
        );
        assert_eq!(
            Decimal::new(7512700, DEFAULT_DECIMALS),
            get_decimal(&data, &BoeConcept::Appraisal)
        );
    }

    #[test]
    fn auction_new_test() {
        let auction: HashMap<BoeConcept, String> = [
            (BoeConcept::Identifier, String::from("SUB-NE-2020-465937")),
            (
                BoeConcept::AuctionKind,
                String::from("NOTARIAL EN VENTA EXTRAJUDICIAL"),
            ),
            (
                BoeConcept::StartDate,
                String::from("14-07-2020 18:00:00 CET  (ISO: 2020-07-14T18:00:00+02:00)"),
            ),
            (
                BoeConcept::EndDate,
                String::from("03-08-2020 18:00:00 CET  (ISO: 2020-08-03T18:00:00+02:00)"),
            ),
            (BoeConcept::ClaimQuantity, String::from("81.971,57 €")),
            (BoeConcept::Lots, String::from("Sin lotes")),
            (BoeConcept::Notice, String::from("BOE-B-2020-21708")),
            (BoeConcept::AuctionValue, String::from("75.127,00 €")),
            (BoeConcept::Appraisal, String::from("75.127,00 €")),
            (BoeConcept::MinimumBid, String::from("SIN PUJA MÍNIMA")),
            (BoeConcept::BidStep, String::from("SIN TRAMOS")),
            (BoeConcept::DepositAmount, String::from("3.756,35 €")),
        ]
        .iter()
        .cloned()
        .collect();

        let mgm = Management {
            code: String::from("3003000230"),
            description: String::from("UNIDAD SUBASTAS JUDICIALES MURCIA (MINISTERIO DE JUSTICIA)"),
            address: String::from("AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA"),
            telephone: String::from("968833360"),
            fax: String::from("-"),
            email: String::from("SUBASTAS.MURCIA@JUSTICIA.ES"),
        };

        let bid = BidInfo {
            appraisal: Decimal::new(75_127_00, DEFAULT_DECIMALS),
            bid_step: Decimal::new(0, DEFAULT_DECIMALS),
            claim_quantity: Decimal::new(81_971_57, DEFAULT_DECIMALS),
            deposit: Decimal::new(375_635, DEFAULT_DECIMALS),
            minimum_bid: Decimal::new(0, DEFAULT_DECIMALS),
            value: Decimal::new(7_512_700, DEFAULT_DECIMALS),
        };

        let auc = Auction {
            id: String::from("SUB-NE-2020-465937"),
            kind: AuctionKind::NotaryExtraJudicial,
            claim_quantity: Decimal::new(8_197_157, DEFAULT_DECIMALS),
            lots: 0,
            lot_kind: LotAuctionKind::NotApplicable,
            management: mgm,
            bidinfo: bid,
            start_date: NaiveDate::parse_from_str("14-07-2020", "%d-%m-%Y").unwrap(),
            end_date: NaiveDate::parse_from_str("03-08-2020", "%d-%m-%Y").unwrap(),
            notice: String::from("BOE-B-2020-21708"),
        };

        let mgm = Management {
            code: String::from("3003000230"),
            description: String::from("UNIDAD SUBASTAS JUDICIALES MURCIA (MINISTERIO DE JUSTICIA)"),
            address: String::from("AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA"),
            telephone: String::from("968833360"),
            fax: String::from("-"),
            email: String::from("SUBASTAS.MURCIA@JUSTICIA.ES"),
        };

        assert_eq!(auc, Auction::new(&auction, mgm));
    }

    #[test]
    fn management_new_test() {
        let mgm: HashMap<BoeConcept, String> = [
            (BoeConcept::Code, String::from("3003000230")),
            (
                BoeConcept::Description,
                String::from("UNIDAD SUBASTAS JUDICIALES MURCIA (MINISTERIO DE JUSTICIA)"),
            ),
            (
                BoeConcept::Address,
                String::from("AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA"),
            ),
            (BoeConcept::Telephone, String::from("968833360")),
            (BoeConcept::Fax, String::from("-")),
            (
                BoeConcept::Email,
                String::from("SUBASTAS.MURCIA@JUSTICIA.ES"),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let management = Management {
            code: String::from("3003000230"),
            description: String::from("UNIDAD SUBASTAS JUDICIALES MURCIA (MINISTERIO DE JUSTICIA)"),
            address: String::from("AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA"),
            telephone: String::from("968833360"),
            fax: String::from("-"),
            email: String::from("SUBASTAS.MURCIA@JUSTICIA.ES"),
        };

        assert_eq!(management, Management::new(&mgm));
    }

    #[test]
    fn asset_new_property_test() {
        let asset_property_map: HashMap<BoeConcept, String> = [
      (
        BoeConcept::Header,
        String::from("BIEN 1 - INMUEBLE (VIVIENDA)"),
      ),
      (
        BoeConcept::Description,
        String::from(
          "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, BAJO-1º",
        ),
      ),
      (
        BoeConcept::CatastroReference,
        String::from("4110202UM5141A0003HH"),
      ),
      (
        BoeConcept::Address,
        String::from("CALLE MARIANO DE LOS COBOS 90"),
      ),
      (BoeConcept::PostalCode, String::from("47014")),
      (BoeConcept::City, String::from("VALLADOLID")),
      (
        BoeConcept::Province,
        String::from("VALLADOLID"),
      ),
      (
        BoeConcept::PrimaryResidence,
        String::from("SÍ"),
      ),
      (
        BoeConcept::OwnerStatus,
        String::from("NO CONSTA"),
      ),
      (
        BoeConcept::Visitable,
        String::from("NO CONSTA"),
      ),
      (
        BoeConcept::RegisterInscription,
        String::from("CONSTA EN EL EDICTO"),
      ),
    ]
    .iter()
    .cloned()
    .collect();

        let id = "id";

        let asset_property = Asset::Property(Property {
            address: String::from("CALLE MARIANO DE LOS COBOS 90"),
            auction_id: id.to_string(),
            bidinfo: None,
            catastro_reference: String::from("4110202UM5141A0003HH"),
            category: PropertyCategory::Apartment,
            charges: Decimal::new(0, DEFAULT_DECIMALS),
            city: String::from("VALLADOLID"),
            coordinates: None,
            description: String::from(
                "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM. 90, BAJO-1º",
            ),
            owner_status: String::from("NO CONSTA"),
            postal_code: String::from("47014"),
            primary_residence: String::from("SÍ"),
            province: Province::Valladolid,
            register_inscription: String::from("CONSTA EN EL EDICTO"),
            visitable: String::from("NO CONSTA"),
        });

        assert_eq!(asset_property, Asset::new(id, &asset_property_map));
    }

    #[test]
    fn asset_new_lot_property_test() {
        let asset_property_map: HashMap<BoeConcept, String> = [
      (BoeConcept::AuctionValue, String::from("15.100,00 €")),
      (BoeConcept::DepositAmount, String::from("755,00 €")),
      (BoeConcept::MinimumBid, String::from("SIN PUJA MÍNIMA")),
      (BoeConcept::BidStep, String::from("302,00 €")),
      (
        BoeConcept::Header,
        String::from("BIEN 1 - INMUEBLE (VIVIENDA)"),
      ),
      (
        BoeConcept::Description,
        String::from(
          "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, BAJO-1º",
        ),
      ),
      (
        BoeConcept::CatastroReference,
        String::from("4110202UM5141A0003HH"),
      ),
      (
        BoeConcept::Address,
        String::from("CALLE MARIANO DE LOS COBOS 90"),
      ),
      (BoeConcept::PostalCode, String::from("47014")),
      (BoeConcept::City, String::from("VALLADOLID")),
      (
        BoeConcept::Province,
        String::from("VALLADOLID"),
      ),
      (
        BoeConcept::PrimaryResidence,
        String::from("SÍ"),
      ),
      (
        BoeConcept::OwnerStatus,
        String::from("NO CONSTA"),
      ),
      (
        BoeConcept::Visitable,
        String::from("NO CONSTA"),
      ),
      (
        BoeConcept::RegisterInscription,
        String::from("CONSTA EN EL EDICTO"),
      ),
    ]
    .iter()
    .cloned()
    .collect();

        let id = "id";

        let asset_property = Asset::Property(Property {
            address: String::from("CALLE MARIANO DE LOS COBOS 90"),
            auction_id: id.to_string(),
            bidinfo: Some(BidInfo {
                appraisal: Decimal::new(0, DEFAULT_DECIMALS),
                bid_step: Decimal::new(302_00, DEFAULT_DECIMALS),
                claim_quantity: Decimal::new(0, DEFAULT_DECIMALS),
                deposit: Decimal::new(755_00, DEFAULT_DECIMALS),
                minimum_bid: Decimal::new(0, DEFAULT_DECIMALS),
                value: Decimal::new(15_100_00, DEFAULT_DECIMALS),
            }),
            catastro_reference: String::from("4110202UM5141A0003HH"),
            category: PropertyCategory::Apartment,
            charges: Decimal::new(0, DEFAULT_DECIMALS),
            city: String::from("VALLADOLID"),
            coordinates: None,
            description: String::from(
                "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM. 90, BAJO-1º",
            ),
            owner_status: String::from("NO CONSTA"),
            postal_code: String::from("47014"),
            primary_residence: String::from("SÍ"),
            province: Province::Valladolid,
            register_inscription: String::from("CONSTA EN EL EDICTO"),
            visitable: String::from("NO CONSTA"),
        });

        assert_eq!(asset_property, Asset::new(id, &asset_property_map));
    }

    #[test]
    fn asset_new_vehicle_test() {
        let asset_vehicle_map: HashMap<BoeConcept, String> = [
      (BoeConcept::AuctionValue, String::from("15.100,00 €")),
      (BoeConcept::DepositAmount, String::from("755,00 €")),
      (BoeConcept::MinimumBid, String::from("SIN PUJA MÍNIMA")),
      (BoeConcept::BidStep, String::from("302,00 €")),
      (
        BoeConcept::Header,
        String::from("BIEN 1 - VEHÍCULO (TURISMOS)"),
      ),
      (
        BoeConcept::Description,
        String::from(
          "VEHÍCULO MATRÍCULA 8868CXV, MARCA: AUDI, MODELO A4, Nº BASTIDOR / Nº CHASIS, EN SU CASO: WAUZZZ8E92A267004."
        ),
      ),
      (
        BoeConcept::LicensePlate,
        String::from("8868CXV"),
      ),
      (
        BoeConcept::Brand,
        String::from("AUDI"),
      ),
      (BoeConcept::Model, String::from("A4")),
      (BoeConcept::FrameNumber, String::from("WAUZZZ8E92A267004.")),
      (
        BoeConcept::LicensedDate,
        String::from("2004-07-02"),
      ),
      (
        BoeConcept::Localization,
        String::from("AVDA. SUAREZ INCLAN, 11, PLAZA DE GARAJE 60 33100 - TRUBIA"),
      ),
      (
        BoeConcept::Visitable,
        String::from("SÍ"),
      ),
    ]
    .iter()
    .cloned()
    .collect();

        let id = "id";

        let asset_vehicle = Asset::Vehicle(Vehicle {
            auction_id: id.to_string(),
            bidinfo: Some(BidInfo {
                appraisal: Decimal::new(0, DEFAULT_DECIMALS),
                bid_step: Decimal::new(302_00, DEFAULT_DECIMALS),
                claim_quantity: Decimal::new(0, DEFAULT_DECIMALS),
                deposit: Decimal::new(755_00, DEFAULT_DECIMALS),
                minimum_bid: Decimal::new(0, DEFAULT_DECIMALS),
                value: Decimal::new(15_100_00, DEFAULT_DECIMALS),
            }),
            brand: String::from("AUDI"),
            charges:  Decimal::new(0, DEFAULT_DECIMALS),
            description: String::from(
                "VEHÍCULO MATRÍCULA 8868CXV, MARCA: AUDI, MODELO A4, Nº BASTIDOR / Nº CHASIS, EN SU CASO: WAUZZZ8E92A267004."
              ),
            frame_number: String::from("WAUZZZ8E92A267004."), // Número de bastidor
            licensed_date: NaiveDate::parse_from_str("2004-07-02", "%Y-%m-%d").unwrap(),
            license_plate: String::from("8868CXV"),
            localization: String::from("AVDA. SUAREZ INCLAN, 11, PLAZA DE GARAJE 60 33100 - TRUBIA"),
            model: String::from("A4"),
            subcategory: String::from("TURISMOS"),
            visitable: String::from("SÍ"),
        });

        assert_eq!(asset_vehicle, Asset::new(id, &asset_vehicle_map));
    }

    #[test]
    fn asset_new_other_test() {
        let asset_other_map: HashMap<BoeConcept, String> = [
      (BoeConcept::AuctionValue, String::from("15.100,00 €")),
      (BoeConcept::DepositAmount, String::from("755,00 €")),
      (BoeConcept::MinimumBid, String::from("SIN PUJA MÍNIMA")),
      (BoeConcept::BidStep, String::from("302,00 €")),
      (
        BoeConcept::Header,
        String::from("BIEN 1 - BIEN MUEBLE (OTROS BIENES Y DERECHOS)"),
      ),
      (
        BoeConcept::Description,
        String::from(
          "CONCESION EXPENDEDURIA DE TABACO Y TIMBRE ALMONTE-1, CODIGO 210049, SITA EN LA C/ DEL OCIO 105 DE ALMONTE (HUELVA)"
        ),
      ),
      (
        BoeConcept::Charges,
        String::from("10.347,54 €"),
      ),
      (
        BoeConcept::JudicialTitle,
        String::from("OTROS DERECHOS"),
      ),
      (
        BoeConcept::AdditionalInformation,
        String::from("LAS CONDICIONES DE LA TRASMISIÓN Y LOS REQUISITOS DEL CONCESIONARIO SE ENCUENTRAN REGULADAS EN EL REAL DECRETO 1199/1999, DE 9 DE JULIO, POR EL QUE SE DESARROLLA LA LEY 13/1998, DE 4 DE MAYO, DE ORDENACIÓN DEL MERCADO DE TABACOS Y NORMATIVA TRIBUTARIA, Y SE REGULA EL ESTATUTO CONCESIONAL DE LA RED DE EXPENDURÍAS DE TABACO Y TIMBRE. VER FOTOGRAFÍAS ANEXAS. - LA CONCESIÓN FINALIZA 03/12/2042. - DILIGENCIA DE EMBARGO A FAVOR DE LA AEAT(2111623311338X), CON IMPORTE PENDIENTE A FECHA 17-09-2020 DE 10.347,54€."),
      ),
      (
        BoeConcept::Visitable,
        String::from("SÍ"),
      ),
    ]
    .iter()
    .cloned()
    .collect();
        let id = "id";
        let asset_other = Asset::Other(Other {
            additional_information: String::from("LAS CONDICIONES DE LA TRASMISIÓN Y LOS REQUISITOS DEL CONCESIONARIO SE ENCUENTRAN REGULADAS EN EL REAL DECRETO 1199/1999, DE 9 DE JULIO, POR EL QUE SE DESARROLLA LA LEY 13/1998, DE 4 DE MAYO, DE ORDENACIÓN DEL MERCADO DE TABACOS Y NORMATIVA TRIBUTARIA, Y SE REGULA EL ESTATUTO CONCESIONAL DE LA RED DE EXPENDURÍAS DE TABACO Y TIMBRE. VER FOTOGRAFÍAS ANEXAS. - LA CONCESIÓN FINALIZA 03/12/2042. - DILIGENCIA DE EMBARGO A FAVOR DE LA AEAT(2111623311338X), CON IMPORTE PENDIENTE A FECHA 17-09-2020 DE 10.347,54€."),
            auction_id: id.to_string(),
            bidinfo: Some(BidInfo {
                appraisal: Decimal::new(0, DEFAULT_DECIMALS),
                bid_step: Decimal::new(302_00, DEFAULT_DECIMALS),
                claim_quantity: Decimal::new( 0, DEFAULT_DECIMALS),
                deposit: Decimal::new(755_00, DEFAULT_DECIMALS),
                minimum_bid: Decimal::new(0, DEFAULT_DECIMALS),
                value: Decimal::new(15_100_00, DEFAULT_DECIMALS),
            }),
            charges:  Decimal::new(1034754, DEFAULT_DECIMALS),
            description: String::from(
                "CONCESION EXPENDEDURIA DE TABACO Y TIMBRE ALMONTE-1, CODIGO 210049, SITA EN LA C/ DEL OCIO 105 DE ALMONTE (HUELVA)"
              ),
            judicial_title: String::from("OTROS DERECHOS"),
            subcategory: String::from("OTROS BIENES Y DERECHOS"),
            visitable: String::from("SÍ"),
        });

        assert_eq!(asset_other, Asset::new(id, &asset_other_map));
    }
}
