use crate::concepts::BoeConcept;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) const DEFAULT_DECIMALS: u32 = 2;
const NOT_APPLICABLE: &str = "NA";

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
        let date_str = date_str.replace("/","-");
        match NaiveDate::parse_from_str(&date_str[..], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                match NaiveDate::parse_from_str(&date_str[..], "%d-%m-%Y") {
                    Ok(date) => date,
                    Err(error) => {
                        log::warn!("Unable to parse date {}: {}", &date_str[..], error.to_string());
                        NaiveDate::parse_from_str("01-01-2000", "%d-%m-%Y").unwrap()
                    }
                }
            }
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
    pub code: String,
    description: String,
    address: String,
    telephone: String,
    fax: String,
    email: String,
}

impl Management {
    /// Create a new Management
    pub fn new(data: &HashMap<BoeConcept, String>) -> Management {
        Management {
            code: data
                .get(&BoeConcept::Code)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            description: data
                .get(&BoeConcept::Description)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
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
    pub id: String,
    kind: AuctionKind,
    claim_quantity: Decimal,
    pub lots: u32,
    pub lot_kind: LotAuctionKind,
    management: Management,
    value: Decimal,
    appraisal: Decimal,
    minimum_bid: Decimal,
    start_date: NaiveDate,
    end_date: NaiveDate,
    notice: String,
    bid_step: Decimal,
    deposit: Decimal,
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
            value: get_decimal(data, &BoeConcept::AuctionValue),
            appraisal: get_decimal(data, &BoeConcept::Appraisal),
            management,
            minimum_bid: get_decimal(data, &BoeConcept::MinimumBid),
            start_date: get_date(data, &BoeConcept::StartDate),
            end_date: get_date(data, &BoeConcept::EndDate),
            notice: data
                .get(&BoeConcept::Notice)
                .unwrap_or(&String::from("BOE"))
                .to_string(),
            bid_step: get_decimal(data, &BoeConcept::BidStep),
            deposit: get_decimal(data, &BoeConcept::DepositAmount),
        }
    }
}

/// Property can be any real state property: apartment, garage lot, industrial ...
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Property {
    address: String,
    auction_id: String,
    catastro_reference: String,
    category: String,
    charges: Decimal,
    city: String,
    description: String,
    owner_status: String,
    postal_code: String,
    primary_residence: String,
    province: String,
    register_inscription: String,
    subcategory: String,
    visitable: String,
}

impl Property {
    /// Create a new property asset.
    pub fn new(
        auction: &str,
        category: &str,
        subcategory: &str,
        data: &HashMap<BoeConcept, String>,
    ) -> Property {
        Property {
            address: data
                .get(&BoeConcept::Address)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            auction_id: auction.to_string(),
            catastro_reference: data
                .get(&BoeConcept::CatastroReference)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            category: category.to_string(),
            charges: get_decimal(data, &BoeConcept::Charges),
            city: data
                .get(&BoeConcept::City)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            description: data
                .get(&BoeConcept::Description)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            owner_status: data
                .get(&BoeConcept::OwnerStatus)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            postal_code: data
                .get(&BoeConcept::PostalCode)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            primary_residence: data
                .get(&BoeConcept::PrimaryResidence)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            province: data
                .get(&BoeConcept::Province)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            register_inscription: data
                .get(&BoeConcept::RegisterInscription)
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

/// Any kind of vehicle
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Vehicle {
    auction_id: String,
    brand: String,
    category: String,
    charges: Decimal,
    description: String,
    frame_number: String, // Número de bastidor
    licensed_date: NaiveDate,
    license_plate: String,
    localization: String,
    model: String,
    subcategory: String,
    visitable: String,
}

impl Vehicle {
    /// Create a new vehicle asset.
    pub fn new(
        auction: &str,
        category: &str,
        subcategory: &str,
        data: &HashMap<BoeConcept, String>,
    ) -> Vehicle {
        Vehicle {
            auction_id: auction.to_string(),
            brand: data
                .get(&BoeConcept::Brand)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            category: category.to_string(),
            charges: get_decimal(data, &BoeConcept::Charges),
            description: data
                .get(&BoeConcept::Description)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
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
    additional_information: String,
    auction_id: String,
    category: String,
    charges: Decimal,
    description: String,
    judicial_title: String,
    subcategory: String,
    visitable: String,
}

impl Other {
    /// Create an asset that is not a vehicle or real state property.
    pub fn new(
        auction: &str,
        category: &str,
        subcategory: &str,
        data: &HashMap<BoeConcept, String>,
    ) -> Other {
        Other {
            additional_information: data
                .get(&BoeConcept::AdditionalInformation)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
            auction_id: auction.to_string(),
            category: category.to_string(),
            charges: get_decimal(data, &BoeConcept::Charges),
            description: data
                .get(&BoeConcept::Description)
                .unwrap_or(&String::from(NOT_APPLICABLE))
                .to_string(),
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
            "INMUEBLE" => Asset::Property(Property::new(auction, &category, &subcategory, data)),
            "VEHÍCULO" => Asset::Vehicle(Vehicle::new(auction, &category, &subcategory, data)),
            _ => Asset::Other(Other::new(auction, &category, &subcategory, data)),
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

        let auc = Auction {
            id: String::from("SUB-NE-2020-465937"),
            kind: AuctionKind::NotaryExtraJudicial,
            claim_quantity: Decimal::new(8197157, DEFAULT_DECIMALS),
            lots: 0,
            lot_kind: LotAuctionKind::NotApplicable,
            management: mgm,
            value: Decimal::new(7512700, DEFAULT_DECIMALS),
            appraisal: Decimal::new(7512700, DEFAULT_DECIMALS),
            minimum_bid: Decimal::new(0, DEFAULT_DECIMALS),
            start_date: NaiveDate::parse_from_str("14-07-2020", "%d-%m-%Y").unwrap(),
            end_date: NaiveDate::parse_from_str("03-08-2020", "%d-%m-%Y").unwrap(),
            notice: String::from("BOE-B-2020-21708"),
            bid_step: Decimal::new(0, DEFAULT_DECIMALS),
            deposit: Decimal::new(375635, DEFAULT_DECIMALS),
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
            catastro_reference: String::from("4110202UM5141A0003HH"),
            category: String::from("INMUEBLE"),
            charges: Decimal::new(0, DEFAULT_DECIMALS),
            city: String::from("VALLADOLID"),
            description: String::from(
                "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, BAJO-1º",
            ),
            owner_status: String::from("NO CONSTA"),
            postal_code: String::from("47014"),
            primary_residence: String::from("SÍ"),
            province: String::from("VALLADOLID"),
            register_inscription: String::from("CONSTA EN EL EDICTO"),
            subcategory: String::from("VIVIENDA"),
            visitable: String::from("NO CONSTA"),
        });

        assert_eq!(asset_property, Asset::new(id, &asset_property_map));
    }

    #[test]
    fn asset_new_vehicle_test() {
        let asset_vehicle_map: HashMap<BoeConcept, String> = [
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
            brand: String::from("AUDI"),
            category: String::from("VEHÍCULO"),
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
            category: String::from("BIEN MUEBLE"),
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
