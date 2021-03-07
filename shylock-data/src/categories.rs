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
                let category: String = s.to_uppercase()
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

                match &category[..] {
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
                PROPERTY_CATEGORIES.get(self).unwrap_or(&"Unknown")
            }
        }

        static PROPERTY_CATEGORIES: Lazy<HashMap<PropertyCategory, &str>> = Lazy::new(|| {
            let mut categories: HashMap<PropertyCategory, &str> = HashMap::new();

            $(
                categories.insert(PropertyCategory::$konst, $display);
            )+

            categories
        });

        #[cfg(test)]
        const TEST_PROPERTY_CATEGORIES: &'static [(PropertyCategory, &'static str, &'static str)] = &[
            $(
            (PropertyCategory::$konst, $name, $display),
            )+
        ];

        #[test]
        fn test_parse_property_category() {
            for &(std, name, _) in TEST_PROPERTY_CATEGORIES {
                // Test upper case
                assert_eq!(name.parse::<PropertyCategory>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<PropertyCategory>().unwrap(), std);
            }
        }

        #[test]
        fn test_property_category_name() {
            for &(std, _, display) in TEST_PROPERTY_CATEGORIES {
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

macro_rules! vehicle_categories {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $name:expr, $display:expr);
        )+
    ) => {
        /// Type of provinces
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
        pub enum VehicleCategory {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        impl FromStr for VehicleCategory {
            type Err = InvalidCategory;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let category: String = s.to_uppercase()
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

                match &category[..] {
                    $(
                    $name => Ok(VehicleCategory::$konst) ,
                    )+
                    "" => Ok(VehicleCategory::Car),
                    _ => Err(InvalidCategory::new(s)),
                }
            }
        }

        impl VehicleCategory {
            /// Returns the string representation for this province
            pub fn name(&self) -> &str {
                VEHICLE_CATEGORIES.get(self).unwrap_or(&"Unknown")
            }
        }

        static VEHICLE_CATEGORIES: Lazy<HashMap<VehicleCategory, &str>> = Lazy::new(|| {
            let mut categories: HashMap<VehicleCategory, &str> = HashMap::new();

            $(
                categories.insert(VehicleCategory::$konst, $display);
            )+

            categories
        });

        #[cfg(test)]
        const TEST_VEHICLE_CATEGORIES: &'static [(VehicleCategory, &'static str, &'static str)] = &[
            $(
            (VehicleCategory::$konst, $name, $display),
            )+
        ];

        #[test]
        fn test_parse_vehicle_category() {
            for &(std, name, _) in TEST_VEHICLE_CATEGORIES {
                // Test upper case
                assert_eq!(name.parse::<VehicleCategory>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<VehicleCategory>().unwrap(), std);
            }
        }

        #[test]
        fn test_vehicle_category_name() {
            for &(std, _, display) in TEST_VEHICLE_CATEGORIES {
                assert_eq!(std.name(), display);
            }
        }

        #[test]
        fn test_parse_invalid_vehicle_category() {
            let invalid_category = "non-sense";
            assert_eq!(invalid_category.parse::<VehicleCategory>(), Err(InvalidCategory::new(invalid_category)));
        }
    }
}

vehicle_categories! {
    /// Car
    (Car, "TURISMOS", "Turismo");

    /// Industrial vehicle
    (Industrial, "INDUSTRIALES", "Vehículo industrial");

    /// Unkown
    (Unknown, "UNKNOWN", "Desconocido");

    /// All
    (All, "ALL", "All");
}

macro_rules! other_categories {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $name:expr, $display:expr);
        )+
    ) => {
        /// Type of provinces
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
        pub enum OtherCategory {
            $(
                $(#[$docs])*
                $konst,
            )+
        }

        impl FromStr for OtherCategory {
            type Err = InvalidCategory;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let category: String = s.to_uppercase()
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

                match &category[..] {
                    $(
                    $name => Ok(OtherCategory::$konst) ,
                    )+
                    "" => Ok(OtherCategory::Other),
                    _ => Err(InvalidCategory::new(s)),
                }
            }
        }

        impl OtherCategory {
            /// Returns the string representation for this province
            pub fn name(&self) -> &str {
                OTHER_CATEGORIES.get(self).unwrap_or(&"Unknown")
            }
        }

        static OTHER_CATEGORIES: Lazy<HashMap<OtherCategory, &str>> = Lazy::new(|| {
            let mut categories: HashMap<OtherCategory, &str> = HashMap::new();

            $(
                categories.insert(OtherCategory::$konst, $display);
            )+

            categories
        });

        #[cfg(test)]
        const TEST_OTHER_CATEGORIES: &'static [(OtherCategory, &'static str, &'static str)] = &[
            $(
            (OtherCategory::$konst, $name, $display),
            )+
        ];

        #[test]
        fn test_parse_other_category() {
            for &(std, name, _) in TEST_OTHER_CATEGORIES {
                // Test upper case
                assert_eq!(name.parse::<OtherCategory>().unwrap(), std);

                // Test lower case
                assert_eq!(name.to_lowercase().parse::<OtherCategory>().unwrap(), std);
            }
        }

        #[test]
        fn test_vehicle_other_name() {
            for &(std, _, display) in TEST_OTHER_CATEGORIES {
                assert_eq!(std.name(), display);
            }
        }

        #[test]
        fn test_parse_invalid_other_category() {
            let invalid_category = "non-sense";
            assert_eq!(invalid_category.parse::<OtherCategory>(), Err(InvalidCategory::new(invalid_category)));
        }
    }
}

other_categories! {
    /// Antiques
    (Antiques, "JOYAS,OBRASDEARTEYANTIGÜEDADES", "Joyas, obras de arte y antigüedades");

    /// Other
    (Other, "OTROS", "Otros");

    /// Other rights
    (OtherRights, "OTROSBIENESYDERECHOS", "Otros bienes y derechos");

    /// Plants
    (Plant, "INSTALACIONES", "Instalación");

    /// Tools
    (Tools, "UTENSILIOSYHERRAMIENTAS", "Utensilios y herramientas");

    /// Vessels
    (Vessel, "BUQUES", "Buque");

    /// Unkown
    (Unknown, "UNKNOWN", "Desconocido");

    /// All
    (All, "ALL", "All");
}
