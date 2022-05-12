use crate::default;
use crate::utils;
use reqwest::IntoUrl;
use std::time::Duration;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Server<TURL>
where
    TURL: IntoUrl,
{
    url: TURL,
    lat: f32,
    lon: f32,
    name: String,
    country: String,
    cc: String,
    sponsor: String,
    id: String,
    host: String,
}
#[derive(Debug, Deserialize)]
pub struct ServersList<TURL>
where
    TURL: IntoUrl,
{
    #[serde(rename = "$value")]
    pub servers: Vec<Server<TURL>>,
}

#[derive(Debug, Deserialize)]
pub struct RootServersLit<TURL>
where
    TURL: IntoUrl,
{
    #[serde(rename = "$value")]
    pub settings: ServersList<TURL>,
}

#[derive(Debug, Deserialize)]
pub struct ClientDetails {
    ip: String,
    lat: f32,
    lon: f32,
    isp: String,
    country: String,
}

#[derive(Debug, Deserialize)]
pub struct LocalClient {
    pub client: ClientDetails,
}

pub struct Host<TURL>
where
    TURL: IntoUrl,
{
    pub client: ClientDetails,
    pub n_w_client: reqwest::blocking::Client,
    pub servers_list: ServersList<TURL>,
}

impl<'a, TURL> Host<TURL>
where
    TURL: IntoUrl + Deserialize<'a>,
{
    pub fn from_local_path(url: TURL) -> Result<Self, utils::CustomError> {
        let lc = utils::fetch_local_client(default::HOST_SPEEDTEST_LINK)?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0",
            ),
        );
        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        let servers_list = utils::fetch_servers_list(default::SERVERS_SPEEDTEST_LINK)?;
        Ok(Self {
            client: lc.client,
            n_w_client: reqwest_client,
            servers_list: servers_list,
        })
    }
}

impl<TURL> Server<TURL>
where
    TURL: IntoUrl + From<String>,
{
    pub fn latency_url(&self) -> TURL {
        let url = format!("http://{}/speedtest/latency.txt", self.host);
        url.into()
    }

    pub fn donwload_urls(&self) -> Vec<TURL> {
        default::DOWNLOAD_SIZE
            .iter()
            .map(|x| format!("http://{}/speedtest/random{}x{}.jpg", self.host, x, x).into())
            .collect()
    }
}

#[derive(Debug)]
pub struct Package<T> {
    pub size: T,
    pub time: Duration,
}
