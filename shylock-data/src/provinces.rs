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
            ($konst:ident, $name:expr, $display:expr);
        )+
    ) => {
        /// Type of provinces
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize, sqlx::Type)]
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
                        'É' => 'E',
                        'Í' => 'I',
                        'Ó' => 'O',
                        'Ú' => 'U',
                        _ => x,
                    }).collect();

                match &province[..] {
                    "ALICANTE/ALACANT" => Ok(Province::Alicante),
                    "ARABA/ALAVA" => Ok(Province::Alava),
                    "BIZKAIA" => Ok(Province::Vizcaya),
                    "CASTELLON/CASTELLO" => Ok(Province::Castellon),
                    "GIPUZKOA" => Ok(Province::Guipuzcoa),
                    "GIRONA" => Ok(Province::Gerona),
                    "ILLESBALEARS" => Ok(Province::Baleares),
                    "ILLESBALLEARS" => Ok(Province::Baleares),
                    "LACORUÑA" => Ok(Province::ACorunia),
                    "LLEIDA" => Ok(Province::Lerida),
                    "OURENSE" => Ok(Province::Orense),
                    "VALENCIA/VALÈNCIA" => Ok(Province::Valencia),
                    "VALÈNCIA" => Ok(Province::Valencia),
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
            provinces.insert(Province::$konst, $display);
            )+

            provinces
        });

        #[cfg(test)]
        const TEST_PROVINCES: &'static [(Province, &'static str, &'static str)] = &[
            $(
            (Province::$konst, $name, $display),
            )+
        ];

        #[test]
        fn test_parse_province() {
            for &(std, name, _) in TEST_PROVINCES {
                // Test upper case
                assert_eq!(name.parse::<Province>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<Province>().unwrap(), std);
            }
        }

        #[test]
        fn test_province_name() {
            for &(std, _, display) in TEST_PROVINCES {
                assert_eq!(std.name(), display);
            }
        }

        #[test]
        fn test_parse_invalid_province() {
            let invalid_province = "non-sense";
            assert_eq!(invalid_province.parse::<Province>(), Err(InvalidProvince::new(invalid_province)));
        }
    }
}

auction_provinces! {
    /// A Coruña province
    (ACorunia, "ACORUÑA", "A Coruña");

    /// Alava province
    (Alava, "ALAVA", "Álava");

    /// Albacete province
    (Albacete, "ALBACETE", "Albacete");

    /// Alicante province
    (Alicante, "ALICANTE", "Alicante");

    /// Almería province
    (Almeria, "ALMERIA", "Almería");

    /// Asturias
    (Asturias, "ASTURIAS", "Asturias");

    /// Ávila province
    (Avila, "AVILA", "Ávila");

    /// Badajoz province
    (Badajoz, "BADAJOZ", "Badajoz");

    /// Baleares province
    (Baleares, "BALEARES", "Baleares");

    /// Barcelona province
    (Barcelona, "BARCELONA", "Barcelona");

    /// Burgos province
    (Burgos, "BURGOS", "Burgos");

    /// Cáceres province
    (Caceres, "CACERES", "Cáceres");

    /// Cádiz province
    (Cadiz, "CADIZ", "Cádiz");

    /// Cantabria province
    (Cantabria, "CANTABRIA", "Cantabria");

    /// Castellón province
    (Castellon, "CASTELLON", "Castellón");

    /// Ciudad Real province
    (CiudadReal, "CIUDADREAL", "Ciudad Real");

    /// Córdoba province
    (Cordoba, "CORDOBA", "Córdoba");

    /// Cuenca province
    (Cuenca, "CUENCA", "Cuenca");

    /// Girona province
    (Gerona, "GERONA", "Gerona");

    /// Granada province
    (Granada, "GRANADA", "Granada");

    /// Guadalajara province
    (Guadalajara, "GUADALAJARA", "Guadalajara");

    /// Guipúzcoa province
    (Guipuzcoa, "GUIPUZCOA", "Guipúzcoa");

    /// Huelva province
    (Huelva, "HUELVA", "Huelva");

    /// Huesca province
    (Huesca, "HUESCA", "Huesca");

    /// Jaén province
    (Jaen, "JAEN", "Jaén");

    /// León province
    (Leon, "LEON", "León");

    /// Lleida province
    (Lerida, "LERIDA", "Lérida");

    /// La Rioja province
    (LaRioja, "LARIOJA", "La Rioja");

    /// Lugo province
    (Lugo, "LUGO", "Lugo");

    /// Madrid province
    (Madrid, "MADRID", "Madrid");

    /// Málaga province
    (Malaga, "MALAGA", "Málaga");

    /// Murcia province
    (Murcia, "MURCIA", "Murcia");

    /// Navarra province
    (Navarra, "NAVARRA", "Navarra");

    /// Ourense province
    (Orense, "ORENSE", "Orense");

    /// Palencia province
    (Palencia, "PALENCIA", "Palencia");

    /// Las Palmas province
    (LasPalmas, "LASPALMAS", "Las Palmas");

    /// Pontevedra province
    (Pontevedra, "PONTEVEDRA", "Pontevedra");

    /// Salamanca province
    (Salamanca, "SALAMANCA", "Salamanca");

    /// Santa Cruz de Tenerife province
    (SantaCruzDeTenerife, "SANTACRUZDETENERIFE", "Santa Cruz de Tenerife");

    /// Segovia province
    (Segovia, "SEGOVIA", "Segovia");

    /// Sevilla province
    (Sevilla, "SEVILLA", "Sevilla");

    /// Soria province
    (Soria, "SORIA", "Soria");

    /// Tarragona province
    (Tarragona, "TARRAGONA", "Tarragona");

    /// Teruel province
    (Teruel, "TERUEL", "Teruel");

    ///  Toledo province
    (Toledo, "TOLEDO", "Toledo");

    /// Valencia province
    (Valencia, "VALENCIA", "Valencia");

    /// Valladolid province
    (Valladolid, "VALLADOLID", "Valladolid");

    /// Vizcaya province
    (Vizcaya, "VIZCAYA", "Vizcaya");

    /// Zamora province
    (Zamora, "ZAMORA", "Zamora");

    /// Zaragoza province
    (Zaragoza, "ZARAGOZA", "Zaragoza");

    /// Ceuta autonomous city
    (Ceuta, "CEUTA", "Ceuta");

    /// Melilla autonomous city
    (Melilla, "MELILLA", "Melilla");

    /// Unkown
    (Unknown, "UNKNOWN", "Desconocido");

    /// All
    (All, "ALL", "All");
}
