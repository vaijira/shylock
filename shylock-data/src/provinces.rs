use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::{error::Error, fmt};

/// Indicates if the province is invalid.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidProvince {
    province: String,
}

impl InvalidProvince {
    fn new(province: &str) -> Self {
        InvalidProvince {
            province: province.to_owned(),
        }
    }
}

impl fmt::Display for InvalidProvince {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unkown province: {}", &self.province[..])
    }
}

impl Error for InvalidProvince {}

macro_rules! auction_provinces {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $name:expr);
        )+
    ) => {
        /// Type of provinces
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
        pub enum Province {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        impl FromStr for Province {
            type Err = InvalidProvince;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let province: String = s.to_uppercase()
                   .replace(" ", "")
                   .chars()
                   .map(|x| match x {
                        'Á' => 'A',
                        'É' => 'É',
                        'Í' => 'I',
                        'Ó' => 'O',
                        'Ú' => 'U',
                        _ => x,
                    }).collect();

                match &province[..] {
                    $(
                    $name => Ok(Province::$konst) ,
                    )+
                    _ => Err(InvalidProvince::new(s)),
                }
            }
        }

        impl Province {
            /// Returns the string representation for this province
            pub fn name(&self) -> &str {
                PROVINCES.get(self).unwrap_or(&"Unknown")
            }
        }

        static PROVINCES: Lazy<HashMap<Province, &str>> = Lazy::new(|| {
            let mut provinces: HashMap<Province, &str> = HashMap::new();

            $(
            provinces.insert(Province::$konst, $name);
            )+

            provinces
        });

        #[cfg(test)]
        const TEST_PROVINCES: &'static [(Province, &'static str)] = &[
            $(
            (Province::$konst, $name),
            )+
        ];

        #[test]
        fn test_parse_province() {
            for &(std, name) in TEST_PROVINCES {
                // Test upper case
                assert_eq!(name.parse::<Province>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<Province>().unwrap(), std);
            }
        }

        #[test]
        fn test_province_name() {
            for &(std, name) in TEST_PROVINCES {
                assert_eq!(std.name(), name);
            }
        }

        #[test]
        fn test_parse_invalid_province() {
            let invalid_concept = "non-sense";
            assert_eq!(invalid_concept.parse::<Province>(), Err(InvalidProvince::new(invalid_concept)));
        }
    }
}

auction_provinces! {
    /// A Coruña province
    (ACorunia, "ACORUÑA");

    /// Alava province
    (Alava, "ALAVA");

    /// Albacete province
    (Albacete, "ALBACETE");

    /// Alicante province
    (Alicante, "ALICANTE");

    /// Almería province
    (Almeria, "ALMERIA");

    /// Asturias
    (Asturias, "ASTURIAS");

    /// Ávila province
    (Avila, "AVILA");

    /// Badajoz province
    (Badajoz, "BADAJOZ");

    /// Baleares province
    (Baleares, "BALEARES");

    /// Barcelona province
    (Barcelona, "BARCELONA");

    /// Burgos province
    (Burgos, "BURGOS");

    /// Cáceres province
    (Caceres, "CACERES");

    /// Cádiz province
    (Cadiz, "CADIZ");

    /// Cantabria province
    (Cantabria, "CANTABRIA");

    /// Castellón province
    (Castellon, "CASTELLON");

    /// Ciudad Real province
    (CiudadReal, "CIUDADREAL");

    /// Córdoba province
    (Cordoba, "CORDOBA");

    /// Cuenca province
    (Cuenca, "CUENCA");

    /// Girona province
    (Gerona, "GERONA");

    /// Granada province
    (Granada, "GRANADA");

    /// Guadalajara province
    (Guadalajara, "GUADALAJARA");

    /// Guipúzcoa province
    (Guipuzcoa, "GUIPUZCOA");

    /// Huelva province
    (Huelva, "HUELVA");

    /// Huesca province
    (Huesca, "HUESCA");

    /// Jaén province
    (Jaen, "JAEN");

    /// León province
    (Leon, "LEON");

    /// Lleida province
    (Lerida, "LERIDA");

    /// La Rioja province
    (LaRioja, "LARIOJA");

    /// Lugo province
    (Lugo, "LUGO");

    /// Madrid province
    (Madrid, "MADRID");

    /// Málaga province
    (Malaga, "MALAGA");

    /// Murcia province
    (Murcia, "MURCIA");

    /// Navarra province
    (Navarra, "NAVARRA");

    /// Ourense province
    (Orense, "ORENSE");

    /// Palencia province
    (Palencia, "PALENCIA");

    /// Las Palmas province
    (LasPalmas, "LASPALMAS");

    /// Pontevedra province
    (Pontevedra, "PONTEVEDRA");

    /// Salamanca province
    (Salamanca, "SALAMANCA");

    /// Santa Cruz de Tenerife province
    (SantaCruzDeTenerife, "SANTACRUZDETENERIFE");

    /// Segovia province
    (Segovia, "SEGOVIA");

    /// Sevilla province
    (Sevilla, "SEVILLA");

    /// Soria province
    (Soria, "SORIA");

    /// Tarragona province
    (Tarragona, "TARRAGONA");

    /// Teruel province
    (Teruel, "TERUEL");

    ///  Toledo province
    (Toledo, "TOLEDO");

    /// Valencia province
    (Valencia, "VALENCIA");

    /// Valladolid province
    (Valladolid, "VALLADOLID");

    /// Vizcaya province
    (Vizcaya, "VIZCAYA");

    /// Zamora province
    (Zamora, "ZAMORA");

    /// Zaragoza province
    (Zaragoza, "ZARAGOZA");

    /// Ceuta autonomous city
    (Ceuta, "CEUTA");

    /// Melilla autonomous city
    (Melilla, "MELILLA");

    /// Unkown
    (Unknown, "UNKNOWN");
}
