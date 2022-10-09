use dominator::routing;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Properties,
    Vehicles,
    OtherAssets,
    Home,
}

impl Route {
    // This could use more advanced URL parsing, but it isn't needed
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        match url.hash().as_str() {
            "#/properties" => Route::Properties,
            "#/vehicles" => Route::Vehicles,
            "#/other-assets" => Route::OtherAssets,
            _ => Route::Home,
        }
    }

    pub fn to_url(self) -> &'static str {
        match self {
            Route::Properties => "#/properties",
            Route::Vehicles => "#/vehicles",
            Route::OtherAssets => "#/other-assets",
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
