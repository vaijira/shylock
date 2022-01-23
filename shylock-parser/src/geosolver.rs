use geo_types::Point;
use lazy_static::lazy_static;
use regex::Regex;
use std::{borrow::Cow, thread, time};

use crate::scraper::APP_USER_AGENT;

const NOMINATIN_OSM_URL: &str = "https://nominatim.openstreetmap.org/search.php";

pub(crate) struct GeoSolver {
    client: reqwest::blocking::Client,
}

impl GeoSolver {
    pub(crate) fn new() -> Self {
        GeoSolver {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(10))
                .timeout(std::time::Duration::from_secs(10))
                .user_agent(APP_USER_AGENT)
                .tcp_nodelay(true)
                .tcp_keepalive(std::time::Duration::from_secs(60))
                .cookie_store(true)
                .pool_max_idle_per_host(10)
                .gzip(true)
                .build()
                .unwrap(),
        }
    }

    fn get_url(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.client.get(target).send()?.error_for_status()?.text()?;

        Ok(body)
    }

    fn clean_address<'a>(&self, address: &'a str) -> Cow<'a, str> {
        lazy_static! {
            static ref REMOVE_SPACES: Regex = Regex::new("[[:blank:]]+").unwrap();
        }

        REMOVE_SPACES.replace_all(address, " ")
    }

    fn try_url(&self, url: &str) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        // Openstreet map is a free service sleep 1 second to not abuse.
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);

        log::debug!("nominatin url: {}", url);
        let body = self.get_url(url)?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        log::debug!("json: {}", json);
        let x = json[0]
            .get("lon")
            .ok_or("no lon field in json")?
            .as_str()
            .ok_or("lon is not a str")?
            .parse::<f64>()?;

        let y = json[0]
            .get("lat")
            .ok_or("no lat field in json")?
            .as_str()
            .ok_or("lat is not a str")?
            .parse::<f64>()?;

        log::debug!("Coordinates x: {}, y: {}", x, y);
        Ok(Some(Point::new(x, y)))
    }
    pub(crate) fn resolve(
        &self,
        address: &str,
        city: &str,
        province: &str,
        country: &str,
        postal_code: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        let url =
            format!(
            "{}?street={}&city={}&state={}&country={}&postalcode={}&countrycodes=es&format=jsonv2",
            NOMINATIN_OSM_URL, self.clean_address(address), city, province, country, postal_code
        );

        let mut result = self.try_url(&url).unwrap_or(None);
        if result.is_none() {
            let url = format!(
                "{}?city={}&state={}&country={}&postalcode={}&countrycodes=es&format=jsonv2",
                NOMINATIN_OSM_URL, city, province, country, postal_code
            );
            result = self.try_url(&url)?;
        }

        Ok(result)
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn deserde_response_test() {
        let body = r#"[{"place_id":256937949,"licence":"Data © OpenStreetMap contributors, ODbL 1.0. https://osm.org/copyright","osm_type":"relation","osm_id":344017,"boundingbox":["39.4567758","39.5696336","-3.1978208","-3.0323874"],"lat":"39.5260507","lon":"-3.076188","display_name":"Miguel Esteban, Mancha Alta de Toledo, Toledo, Castile-La Mancha, Spain","place_rank":16,"category":"boundary","type":"administrative","importance":0.8968876506482677,"icon":"https://nominatim.openstreetmap.org/ui/mapicons//poi_boundary_administrative.p.20.png"}]"#;
        let json: serde_json::Value =
            serde_json::from_str(body).expect("JSON was not well-formatted");
        assert_eq!(json[0].get("lat").unwrap().as_str(), Some("39.5260507"));
        assert_eq!(json[0].get("lon").unwrap().as_str(), Some("-3.076188"));
    }
}
