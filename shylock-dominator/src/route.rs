use dominator::routing;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Properties,
    PropertiesMap,
    Vehicles,
    OtherAssets,
    Statistics,
    Home,
}

impl Route {
    // This could use more advanced URL parsing, but it isn't needed
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        match url.hash().as_str() {
            "#/properties" => Route::Properties,
            "#/properties-map" => Route::PropertiesMap,
            "#/vehicles" => Route::Vehicles,
            "#/other-assets" => Route::OtherAssets,
            "#/statistics" => Route::Statistics,
            _ => Route::Home,
        }
    }

    pub fn to_url(self) -> &'static str {
        match self {
            Route::Properties => "#/properties",
            Route::PropertiesMap => "#/properties-map",
            Route::Vehicles => "#/vehicles",
            Route::OtherAssets => "#/other-assets",
            Route::Statistics => "#/statistics",
            Route::Home => "#/",
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        // Create the Route based on the current URL
        Self::from_url(&routing::url().lock_ref())
    }
}
