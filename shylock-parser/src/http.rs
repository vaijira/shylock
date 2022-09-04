use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

pub(crate) const BASE_BOE_URL: &str = "https://subastas.boe.es/";

lazy_static! {
    /// URI that to obtain all ongoing auctions in BOE page.
    pub static ref MAIN_ONGOING_AUCTIONS_BOE_URL: String = BASE_BOE_URL.to_owned() + "subastas_ava.php?campo%5B0%5D=SUBASTA.ORIGEN&dato%5B0%5D=&campo%5B1%5D=SUBASTA.AUTORIDAD&dato%5B1%5D=&campo%5B2%5D=SUBASTA.ESTADO&dato%5B2%5D=EJ&campo%5B3%5D=BIEN.TIPO&dato%5B3%5D=&dato%5B4%5D=&campo%5B5%5D=BIEN.DIRECCION&dato%5B5%5D=&campo%5B6%5D=BIEN.CODPOSTAL&dato%5B6%5D=&campo%5B7%5D=BIEN.LOCALIDAD&dato%5B7%5D=&campo%5B8%5D=BIEN.COD_PROVINCIA&dato%5B8%5D=&campo%5B9%5D=SUBASTA.POSTURA_MINIMA_MINIMA_LOTES&dato%5B9%5D=&campo%5B10%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_1&dato%5B10%5D=&campo%5B11%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_2&dato%5B11%5D=&campo%5B12%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_3&dato%5B12%5D=&campo%5B13%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_4&dato%5B13%5D=&campo%5B14%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_5&dato%5B14%5D=&campo%5B15%5D=SUBASTA.ID_SUBASTA_BUSCAR&dato%5B15%5D=&campo%5B16%5D=SUBASTA.FECHA_FIN_YMD&dato%5B16%5D%5B0%5D=&dato%5B16%5D%5B1%5D=&campo%5B17%5D=SUBASTA.FECHA_INICIO_YMD&dato%5B17%5D%5B0%5D=&dato%5B17%5D%5B1%5D=&page_hits=500&sort_field%5B0%5D=SUBASTA.FECHA_FIN_YMD&sort_order%5B0%5D=desc&sort_field%5B1%5D=SUBASTA.FECHA_FIN_YMD&sort_order%5B1%5D=asc&sort_field%5B2%5D=SUBASTA.HORA_FIN&sort_order%5B2%5D=asc&accion=Buscar";
    /// URI that to obtain all auctions in BOE page.
    pub static ref MAIN_ALL_AUCTIONS_BOE_URL: String = BASE_BOE_URL.to_owned() + "subastas_ava.php?campo%5B0%5D=SUBASTA.ORIGEN&dato%5B0%5D=&campo%5B1%5D=SUBASTA.AUTORIDAD&dato%5B1%5D=&campo%5B2%5D=SUBASTA.ESTADO&dato%5B2%5D=&campo%5B3%5D=BIEN.TIPO&dato%5B3%5D=&dato%5B4%5D=&campo%5B5%5D=BIEN.DIRECCION&dato%5B5%5D=&campo%5B6%5D=BIEN.CODPOSTAL&dato%5B6%5D=&campo%5B7%5D=BIEN.LOCALIDAD&dato%5B7%5D=&campo%5B8%5D=BIEN.COD_PROVINCIA&dato%5B8%5D=&campo%5B9%5D=SUBASTA.POSTURA_MINIMA_MINIMA_LOTES&dato%5B9%5D=&campo%5B10%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_1&dato%5B10%5D=&campo%5B11%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_2&dato%5B11%5D=&campo%5B12%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_3&dato%5B12%5D=&campo%5B13%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_4&dato%5B13%5D=&campo%5B14%5D=SUBASTA.NUM_CUENTA_EXPEDIENTE_5&dato%5B14%5D=&campo%5B15%5D=SUBASTA.ID_SUBASTA_BUSCAR&dato%5B15%5D=&campo%5B16%5D=SUBASTA.FECHA_FIN_YMD&dato%5B16%5D%5B0%5D=&dato%5B16%5D%5B1%5D=&campo%5B17%5D=SUBASTA.FECHA_INICIO_YMD&dato%5B17%5D%5B0%5D=&dato%5B17%5D%5B1%5D=&page_hits=500&sort_field%5B0%5D=SUBASTA.FECHA_FIN_YMD&sort_order%5B0%5D=asc&sort_field%5B1%5D=SUBASTA.FECHA_FIN_YMD&sort_order%5B1%5D=asc&sort_field%5B2%5D=SUBASTA.HORA_FIN&sort_order%5B2%5D=asc&accion=Buscar";
}

/// Name of the user agent used in http requests
pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Http client to make requests to BOE website.
pub trait HttpClient {
    /// Get the content of the url
    fn get_url(&self, target: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub(crate) struct BlockingUrlFetcher {
    client: reqwest::blocking::Client,
}

impl BlockingUrlFetcher {
    pub(crate) fn new() -> Self {
        BlockingUrlFetcher {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(10))
                .timeout(std::time::Duration::from_secs(10))
                .user_agent(APP_USER_AGENT)
                .cookie_store(true)
                .tcp_nodelay(true)
                .tcp_keepalive(std::time::Duration::from_secs(60))
                .pool_max_idle_per_host(10)
                .gzip(true)
                .build()
                .unwrap(),
        }
    }
}

impl HttpClient for BlockingUrlFetcher {
    fn get_url(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut retries = 3;
        while retries > 0 {
            match self.client.get(target).send() {
                Ok(value) => {
                    let body = value.error_for_status()?.text()?;
                    return Ok(body);
                }
                Err(value) => {
                    retries -= 1;
                    if retries > 0 {
                        continue;
                    } else {
                        return Err(Box::new(value));
                    }
                }
            }
        }

        Ok("".to_string())
    }
}

/// HTTP client.
#[derive(Debug)]
pub struct UrlFetcher {
    client: ClientWithMiddleware,
}

impl UrlFetcher {
    /// Create new http client with default options.
    pub fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(APP_USER_AGENT)
            .cookie_store(true)
            .tcp_nodelay(true)
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .gzip(true)
            .build()
            .unwrap();

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(5);

        UrlFetcher {
            client: ClientBuilder::new(http_client)
                // Trace HTTP requests. See the tracing crate to make use of these traces.
                .with(TracingMiddleware)
                // Retry failed requests.
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
        }
    }

    /// Returns `target` web page content or return errors if unable.
    pub async fn get_url(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.get(target).send().await?;
        let body = response.error_for_status()?.text().await?;
        Ok(body)
    }
}

impl Default for UrlFetcher {
    fn default() -> Self {
        Self::new()
    }
}
