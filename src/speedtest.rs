use reqwest::IntoUrl;

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
    client: ClientDetails,
}

impl<TURL> Server<TURL>
where
    TURL: IntoUrl + From<String>,
{
    pub fn latency_url(&self) -> TURL {
        let url = format!("http://{}/speedtest/latency.txt", self.host);
        url.into()
    }
}
