use std::str::FromStr;
use std::{error::Error, fmt};

/// Indicates if the concept is invalid.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidBoeConcept {
    concept: String,
}

impl InvalidBoeConcept {
    fn new(concept: &str) -> Self {
        InvalidBoeConcept {
            concept: concept.to_owned(),
        }
    }
}

impl fmt::Display for InvalidBoeConcept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unkown BOE concept: {}", &self.concept[..])
    }
}

impl Error for InvalidBoeConcept {}

macro_rules! boe_auction_concepts {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $name:expr);
        )+
    ) => {
        /// Type of BOE concepts
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub enum BoeConcept {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        impl FromStr for BoeConcept {
            type Err = InvalidBoeConcept;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match &s.to_uppercase()[..] {
                    $(
                    $name => Ok(BoeConcept::$konst) ,
                    )+
                    "FORMA ADJUDICACIÓN" => Ok(BoeConcept::LotAuctionKind),
                    "VALOR DE TASACIÓN" => Ok(BoeConcept::Appraisal),
                    "FECHA DE ADQUISICIÓN" => Ok(BoeConcept::AcquisitionDate),
                    "FECHA DE MATRICULACIÓN" => Ok(BoeConcept::LicensedDate),
                    "REFERENCIA REGISTRAL" => Ok(BoeConcept::RegisterInscription),
                    "NOMBRE PARAJE" => Ok(BoeConcept::Place),
                    _ => Err(InvalidBoeConcept::new(s)),
                }
            }
        }

        #[cfg(test)]
        const TEST_BOE_CONCEPTS: &'static [(BoeConcept, &'static str)] = &[
            $(
            (BoeConcept::$konst, $name),
            )+
        ];

        #[test]
        fn test_parse_boe_concepts() {
            for &(std, name) in TEST_BOE_CONCEPTS {
                // Test upper case
                assert_eq!(name.parse::<BoeConcept>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<BoeConcept>().unwrap(), std);
            }
        }

        #[test]
        fn test_parse_invalid_boe_concept() {
            let invalid_concept = "non-sense";
            assert_eq!(invalid_concept.parse::<BoeConcept>(), Err(InvalidBoeConcept::new(invalid_concept)));
        }
    }
}

boe_auction_concepts! {
    /// Acquisition date
    (AcquisitionDate, "FECHA ADQUISICIÓN");

    /// Additional information
    (AdditionalInformation, "INFORMACIÓN ADICIONAL");

    /// Specify physical address of the resource.
    (Address, "DIRECCIÓN");

    /// Allotment
    (Allotment, "PARCELA");

    /// Value of the asset according to official authorities.
    (Appraisal, "TASACIÓN");

    /// Area of a property asset
    (Area, "SUPERFICIE");

    /// Specify the kind of auction, so far:
    /// * AGENCIA TRIBUTARIA
    /// * RECAUDACIÓN TRIBUTARIA
    /// * NOTARIAL VOLUNTARIA
    /// * JUDICIAL VOLUNTARIA
    /// * JUDICIAL EN VIA DE APREMIO
    /// * JUDICIAL CONCURSAL
    /// * NOTARIAL EN VENTA EXTRAJUDICIAL
    /// * Desconocida,
    (AuctionKind, "TIPO DE SUBASTA");

    /// Auction value
    (AuctionValue, "VALOR SUBASTA");

    /// Steps between bids
    (BidStep, "TRAMOS ENTRE PUJAS");

    /// Vehicle brand
    (Brand, "MARCA");

    /// Charges
    (Charges, "CARGAS");

    /// Catastro reference, we can go to the official catastro webpage to see
    /// everything related with the Residence.
    (CatastroReference, "REFERENCIA CATASTRAL");

    /// City
    (City, "LOCALIDAD");

    /// The quantity of the claim by the creditors.
    (ClaimQuantity, "CANTIDAD RECLAMADA");

    /// Code of the auction
    (Code, "CÓDIGO");

    /// Deposit amount to be able to make bids in the auction.
    (DepositAmount, "IMPORTE DEL DEPÓSITO");

    /// Description of the concept.
    (Description, "DESCRIPCIÓN");

    /// Email.
    (Email, "CORREO ELECTRÓNICO");

    /// End date of the auction.
    (EndDate, "FECHA DE CONCLUSIÓN");

    /// Fax.
    (Fax, "FAX");

    /// Frame number.
    (FrameNumber, "NÚMERO DE BASTIDOR");

    /// Header concept.
    (Header, "HEADER");

    /// Identifier of the concept.
    (Identifier, "IDENTIFICADOR");

    /// IDUFIR (Identificador único de finca registral)
    (Idufir, "IDUFIR");

    /// Judicial title.
    (JudicialTitle, "TÍTULO JURÍDICO");

    /// Licensed date.
    (LicensedDate, "FECHA MATRICULACIÓN");

    /// License plate.
    (LicensePlate, "MATRÍCULA");

    /// Asset localization.
    (Localization, "DEPÓSITO");

    /// It specifies in this concept contains lots or not.
    (Lots, "LOTES");

    /// Specify if the lots are joined or splitted.
    (LotAuctionKind, "FORMA DE ADJUDICACIÓN");

    /// Minimum bid to be able to access the auction.
    (MinimumBid, "PUJA MÍNIMA");

    /// Vehicle model.
    (Model, "MODELO");

    /// BOE Notice.
    (Notice, "ANUNCIO BOE");

    /// Owner status of the asset.
    (OwnerStatus, "SITUACIÓN POSESORIA");

    /// Place
    (Place, "PARAJE");

    /// Postal code.
    (PostalCode, "CÓDIGO POSTAL");

    /// Province.
    (Province, "PROVINCIA");

    /// Primary residence.
    (PrimaryResidence, "VIVIENDA HABITUAL");

    /// Quota
    (Quota, "CUOTA");

    /// Register inscription.
    (RegisterInscription, "INSCRIPCIÓN REGISTRAL");

    /// Start date of the auction.
    (StartDate, "FECHA DE INICIO");

    /// Telephone.
    (Telephone, "TELÉFONO");

    /// If the asset can be viewed or not.
    (Visitable, "VISITABLE");
}
