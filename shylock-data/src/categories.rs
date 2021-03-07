use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::{error::Error, fmt};

/// Indicates if the province is invalid.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCategory {
    category: String,
}

impl InvalidCategory {
    fn new(category: &str) -> Self {
        InvalidCategory {
            category: category.to_owned(),
        }
    }
}

impl fmt::Display for InvalidCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unkown category: {}", &self.category[..])
    }
}

impl Error for InvalidCategory {}

macro_rules! property_categories {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $name:expr, $display:expr);
        )+
    ) => {
        /// Type of provinces
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
        pub enum PropertyCategory {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        impl FromStr for PropertyCategory {
            type Err = InvalidCategory;

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
                    $(
                    $name => Ok(PropertyCategory::$konst) ,
                    )+
                    "" => Ok(PropertyCategory::Apartment),
                    _ => Err(InvalidCategory::new(s)),
                }
            }
        }

        impl PropertyCategory {
            /// Returns the string representation for this province
            pub fn name(&self) -> &str {
                CATEGORIES.get(self).unwrap_or(&"Unknown")
            }
        }

        static CATEGORIES: Lazy<HashMap<PropertyCategory, &str>> = Lazy::new(|| {
            let mut categories: HashMap<PropertyCategory, &str> = HashMap::new();

            $(
                categories.insert(PropertyCategory::$konst, $display);
            )+

            categories
        });

        #[cfg(test)]
        const TEST_CATEGORIES: &'static [(PropertyCategory, &'static str, &'static str)] = &[
            $(
            (PropertyCategory::$konst, $name, $display),
            )+
        ];

        #[test]
        fn test_parse_property_category() {
            for &(std, name, _) in TEST_CATEGORIES {
                // Test upper case
                assert_eq!(name.parse::<PropertyCategory>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<PropertyCategory>().unwrap(), std);
            }
        }

        #[test]
        fn test_property_category_name() {
            for &(std, _, display) in TEST_CATEGORIES {
                assert_eq!(std.name(), display);
            }
        }

        #[test]
        fn test_parse_invalid_property_category() {
            let invalid_category = "non-sense";
            assert_eq!(invalid_category.parse::<PropertyCategory>(), Err(InvalidCategory::new(invalid_category)));
        }
    }
}

property_categories! {
    /// Apartment
    (Apartment, "VIVIENDA", "Vivienda");

    /// Building site
    (BuildingSite, "SOLAR", "Solar");

    /// Business premises
    (BusinessPremises, "LOCALCOMERCIAL", "Local comercial");

    /// Garage
    (Garage, "GARAJE", "Garaje");

    /// Industrial estate
    (IndustrialState, "NAVEINDUSTRIAL", "Nave industrial");

    /// Rustica estate
    (RusticState, "FINCARUSTICA", "Finca rústica");

    /// Storage room
    (StorageRoom, "TRASTERO", "Trastero");

    /// Unkown
    (Unknown, "UNKNOWN", "Desconocido");

    /// All
    (All, "ALL", "All");
}
