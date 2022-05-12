use crate::speedtest::{Host, LocalClient, Package, RootServersLit, Server, ServersList};
use rand::random;
use reqwest::IntoUrl;
use serde::Deserialize;
use serde_xml_rs;
use std::fmt::{Debug, Display};
use std::time::{Duration, Instant};
#[derive(Debug)]
pub struct CustomError {
    _from: String,
    _status_code: Option<u16>,
    _message: String,
}

impl CustomError {
    fn new(_status_code: Option<u16>, _message: String, _from: String) -> Self {
        Self {
            _from,
            _status_code,
            _message,
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        let status_code = match error.status() {
            Some(x) => Some(x.as_u16()),
            None => None,
        };
        let message = error.to_string();
        Self::new(status_code, message, "reqwest".into())
    }
}

impl From<serde_xml_rs::Error> for CustomError {
    fn from(error: serde_xml_rs::Error) -> Self {
        let message = error.to_string();
        let status_code = None;
        Self::new(status_code, message, "xml".into())
    }
}

pub fn fetch_servers_list<'a, TURL>(url: &str) -> Result<ServersList<TURL>, CustomError>
where
    TURL: IntoUrl + Deserialize<'a>,
{
    let response = reqwest::blocking::get(url)?.text()?;
    let server_list = serde_xml_rs::from_str::<RootServersLit<TURL>>(&response)?;
    Ok(server_list.settings)
}

pub fn fetch_local_client(url: &str) -> Result<LocalClient, CustomError> {
    let response = reqwest::blocking::get(url)?.text()?;
    let localclient = serde_xml_rs::from_str::<LocalClient>(&response)?;
    Ok(localclient)
}

pub trait URLExtension
where
    Self: Sized,
    String: From<Self>,
    Self: From<String>,
{
    fn cache_solve_url(self) -> Self {
        let url: String = self.into();
        let url = format!("{}?x={}", url, random::<f32>());
        url.into()
    }
}

impl URLExtension for String {}

pub trait NetworkTest<TURL>
where
    TURL: IntoUrl + From<String> + Display,
{
    fn n_client(&self) -> &reqwest::blocking::Client;
    fn ping(&self, url: TURL) -> Result<Package<usize>, CustomError> {
        println!("url: {}", url);
        let rb = self.n_client().get(url);
        let now = Instant::now();
        let size = rb.send()?.bytes()?.len();
        let elaspsed = now.elapsed();
        println!("bytes:{} - duration:{:?}", size, elaspsed);
        Ok(Package {
            size,
            time: elaspsed,
        })
    }

    fn shortest_server<'a>(
        &self,
        servers_list: &'a ServersList<TURL>,
    ) -> (Package<usize>, &'a Server<TURL>)
    where
        String: From<TURL>,
        TURL: URLExtension,
    {
        let mut array = servers_list
            .servers
            .iter()
            .map(|x| {
                let url = x.latency_url();
                let url = url.cache_solve_url();
                self.ping(url).unwrap()
            })
            .zip(servers_list.servers.iter())
            .collect::<Vec<(Package<usize>, &Server<TURL>)>>();
        array.sort_by_key(|x| x.0.time);
        array.remove(0)
    }

    fn download_test(&self, server: &Server<TURL>) -> Package<f32>
    where
        String: From<TURL>,
        TURL: URLExtension,
    {
        let urls = server.donwload_urls();
        let count = urls.len();
        let package = urls
            .into_iter()
            .map(|x| {
                let url = x.cache_solve_url();
                self.ping(url).unwrap()
            })
            .reduce(|x, y| Package {
                size: x.size + y.size,
                time: x.time + y.time,
            })
            .unwrap();
        Package {
            // size: (package.size * 8) as f32 / (count * 1000000) as f32,
            size: package.size as f32 / count as f32,
            time: package.time / count as u32,
        }
    }
}

impl<TURL> NetworkTest<TURL> for Host<TURL>
where
    TURL: IntoUrl + From<String> + Display,
{
    fn n_client(&self) -> &reqwest::blocking::Client {
        &self.n_w_client
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch_servers_list, Host, NetworkTest};
    use crate::default;

    #[test]
    fn test_server_list() {
        let _config = fetch_servers_list::<String>(default::SERVERS_SPEEDTEST_LINK).unwrap();
    }
    #[test]
    fn test_ping() {
        let lc = Host::from_local_path(default::HOST_SPEEDTEST_LINK.to_string()).unwrap();

        let duration = lc.ping(
            "http://testspeed.vainavi.net:8080/speedtest/latency.txt?x=1652247799206.0".to_string(),
        );
        println!("{:?}", duration)
    }

    #[test]
    fn get_best_server() {
        let lc = Host::from_local_path(default::HOST_SPEEDTEST_LINK.to_string()).unwrap();
        let shortest_server = lc.shortest_server(&lc.servers_list);
        println!("{:?}", shortest_server)
    }

    #[test]
    fn speed_test() {
        let lc = Host::from_local_path(default::HOST_SPEEDTEST_LINK.to_string()).unwrap();
        let shortest_server = lc.shortest_server(&lc.servers_list);
        let pkg = lc.download_test(shortest_server.1);
        println!("{:?}", pkg);
        println!(
            "{:?}",
            8 as f32 * (pkg.size / 1000000 as f32) / pkg.time.as_secs_f32()
        )
    }

    #[test]
    fn get_bytes() {
        let url = "http://speedtest.actcorp.in:8080/speedtest/random350x350.jpg";
        let client = reqwest::blocking::Client::new().get(url).header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0",
        );
        let response = client.send().unwrap();
        let x = response.bytes().unwrap().len();
        println!("{:?}", x)
    }
}
