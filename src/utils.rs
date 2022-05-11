use crate::speedtest::{LocalClient, RootServersLit, Server, ServersList};
use reqwest::IntoUrl;
use serde::Deserialize;
use serde_xml_rs;
use std::fmt::Debug;
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

pub trait NetworkTest<TURL>
where
    TURL: IntoUrl + From<String>,
{
    fn ping(&self, url: TURL) -> Result<Duration, CustomError> {
        let now = Instant::now();
        let response = reqwest::blocking::get(url)?.text()?;
        Ok(now.elapsed())
    }

    fn shortest_server<'a>(
        &self,
        servers_list: &'a ServersList<TURL>,
    ) -> (Duration, &'a Server<TURL>) {
        let mut array = servers_list
            .servers
            .iter()
            .map(|x| self.ping(x.latency_url()).unwrap())
            .zip(servers_list.servers.iter())
            .collect::<Vec<(Duration, &Server<TURL>)>>();
        array.sort_by_key(|&x| x.0);
        array[0]
    }
}

impl<TURL> NetworkTest<TURL> for LocalClient where TURL: IntoUrl + From<String> {}

#[cfg(test)]
mod tests {
    use super::{fetch_local_client, fetch_servers_list, NetworkTest};
    use crate::default;

    #[test]
    fn test_server_list() {
        let _config = fetch_servers_list::<String>(default::SERVERS_SPEEDTEST_LINK).unwrap();
    }
    #[test]
    fn test_ping() {
        let lc = fetch_local_client(default::HOST_SPEEDTEST_LINK).unwrap();
        let duration = lc.ping(
            "http://testspeed.vainavi.net:8080/speedtest/latency.txt?x=1652247799206.0".to_string(),
        );
        println!("{:?}", duration)
    }

    #[test]
    fn get_best_server() {
        let lc = fetch_local_client(default::HOST_SPEEDTEST_LINK).unwrap();
        let servers_list = fetch_servers_list::<String>(default::SERVERS_SPEEDTEST_LINK).unwrap();
        let shortest_server = lc.shortest_server(&servers_list);
        println!("{:?}", shortest_server)
    }

    #[test]
    fn speed_test() {}
}
