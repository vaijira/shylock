use geo_types::Point;
use lazy_static::lazy_static;
use proj::Proj;
use regex::Regex;
use std::{borrow::Cow, thread, time};

use crate::{
    http::UrlFetcher,
    parser::{
        parse_coordinates_from_catastro_cpmrc_response, parse_data_from_catastro_dnprc_response,
    },
    util::valid_catastro_reference,
};

const NOMINATIN_OSM_URL: &str = "https://nominatim.openstreetmap.org/search.php";

const SRS_EPSG_4326: &str = "EPSG:4326";
const SRS_EPS_3857: &str = "EPSG:3857";

const BASE_CATASTRO_URL: &str = "http://ovc.catastro.meh.es/";

// Catastro specification https://www.catastro.meh.es/ws/Webservices_Libres.pdf
lazy_static! {
    // Web page to test it https://ovc.catastro.meh.es/ovcservweb/OVCSWLocalizacionRC/OVCCallejero.asmx?op=Consulta_DNPRC
    static ref CATASTRO_DATA_QUERY_WITH_REFERENCE_URL: String = BASE_CATASTRO_URL.to_owned() + "/ovcservweb/OVCSWLocalizacionRC/OVCCallejero.asmx/Consulta_DNPRC?Provincia=&Municipio=&RC=";
    // Web page to test it https://ovc.catastro.meh.es/ovcservweb/OVCSWLocalizacionRC/OVCCoordenadas.asmx?op=Consulta_CPMRC
    static ref CATASTRO_COORDINATES_QUERY_WITH_REFERENCE_URL: String = BASE_CATASTRO_URL.to_owned() + &format!("/ovcservweb/OVCSWLocalizacionRC/OVCCoordenadas.asmx/Consulta_CPMRC?Provincia=&Municipio=&SRS={}&RC=", SRS_EPSG_4326);
}

/// Provides client for geosolving addresses.
#[derive(Debug)]
pub struct GeoSolver {
    client: UrlFetcher,
}

impl GeoSolver {
    /// Creates geosolver client.
    pub fn new() -> Self {
        GeoSolver {
            client: UrlFetcher::new(),
        }
    }

    async fn get_url(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.client.get_url(target).await?;

        Ok(body)
    }

    fn transform_cooordinates(&self, point: Point<f64>) -> Point<f64> {
        let coordinate_transformer =
            Proj::new_known_crs(SRS_EPSG_4326, SRS_EPS_3857, None).unwrap();

        coordinate_transformer.convert(point).unwrap()
    }

    fn clean_address<'a>(&self, address: &'a str) -> Cow<'a, str> {
        lazy_static! {
            static ref REMOVE_SPACES: Regex = Regex::new("[[:blank:]]+").unwrap();
        }

        REMOVE_SPACES.replace_all(address, " ")
    }

    async fn try_nominatin_url(
        &self,
        url: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        log::debug!("nominatin url: {}", url);
        let body = self.get_url(url).await?;
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

    async fn try_catastro_reference_url(
        &self,
        url: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        log::debug!("catastro reference url: {}", url);
        let body = self.get_url(url).await?;

        parse_coordinates_from_catastro_cpmrc_response(&body)
    }

    async fn try_nominatin(
        &self,
        address: &str,
        city: &str,
        province: &str,
        country: &str,
        postal_code: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}?street={}&city={}&state={}&country={}&postalcode={}&countrycodes=es&format=jsonv2",
            NOMINATIN_OSM_URL,
            self.clean_address(address),
            city,
            province,
            country,
            postal_code
        );

        let mut result = self.try_nominatin_url(&url).await.unwrap_or(None);
        // Openstreet map is a free service sleep 1 second to not abuse.
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        if result.is_none() {
            let url = format!(
                "{}?city={}&state={}&country={}&postalcode={}&countrycodes=es&format=jsonv2",
                NOMINATIN_OSM_URL, city, province, country, postal_code
            );
            result = self.try_nominatin_url(&url).await?;
            // Openstreet map is a free service sleep 1 second to not abuse.
            thread::sleep(one_second);
        }

        Ok(result)
    }

    async fn try_catastro_reference(
        &self,
        catastro_reference: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        let url = if valid_catastro_reference(catastro_reference) {
            format!(
                "{}{}",
                &*CATASTRO_COORDINATES_QUERY_WITH_REFERENCE_URL,
                &catastro_reference[0..14]
            )
        } else {
            return Ok(None);
        };

        let mut result: Option<Point<f64>> = self.try_catastro_reference_url(&url).await?;
        result = Some(self.transform_cooordinates(result.unwrap()));
        Ok(result)
    }

    /// Return a latitude and longitude point if it resolves successfully
    /// an `address`, `city`, `province`, `country`, `postal_code` and
    /// `catastro_reference` information.
    pub async fn resolve(
        &self,
        address: &str,
        city: &str,
        province: &str,
        country: &str,
        postal_code: &str,
        catastro_reference: &str,
    ) -> Result<Option<Point<f64>>, Box<dyn std::error::Error>> {
        let mut result = self
            .try_catastro_reference(catastro_reference)
            .await
            .unwrap_or(None);

        if result.is_none() {
            result = self
                .try_nominatin(address, city, province, country, postal_code)
                .await?;
        }

        Ok(result)
    }

    /// get catastro link from a `catastro_reference`.
    pub async fn get_catastro_link(
        &self,
        catastro_reference: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let url = if valid_catastro_reference(catastro_reference) {
            format!(
                "{}{}",
                &*CATASTRO_DATA_QUERY_WITH_REFERENCE_URL, catastro_reference
            )
        } else {
            return Ok(None);
        };
        log::debug!("catastro reference url: {}", &url);
        let body = self.get_url(&url).await?;

        parse_data_from_catastro_dnprc_response(&body, catastro_reference)
    }
}

impl Default for GeoSolver {
    fn default() -> Self {
        Self::new()
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
