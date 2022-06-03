use crate::defaults::{self, OOKLA_SPEEDTEST_COMMAND};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::process::Command;
pub struct Ookla;

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadResult {
    download: f64,
    link: String,
    upload: f64,
    timestamp: String,
}
impl Ookla {
    pub fn new() -> Self {
        Ookla
    }
    pub fn test(&self) -> DownloadResult {
        let op = Command::new(OOKLA_SPEEDTEST_COMMAND)
            .arg("-f")
            .arg("json-pretty")
            .output()
            .expect("Command Failed");
        let resp = serde_json::from_slice::<serde_json::Value>(&op.stdout).unwrap();
        let d_bytes = resp
            .get(defaults::DOWNLOAD_KEY)
            .unwrap()
            .get(defaults::BANDWIDRH)
            .unwrap();

        let u_bytes = resp
            .get(defaults::UPLOAD_KEY)
            .unwrap()
            .get(defaults::BANDWIDRH)
            .unwrap();

        let url = resp
            .get(defaults::RESULT)
            .unwrap()
            .get(defaults::URL)
            .unwrap();

        let timestamp = resp.get(defaults::TIMESTAMP).unwrap();

        if let (
            Value::Number(d_b),
            Value::Number(u_b),
            Value::String(x_url),
            Value::String(x_timestamp),
        ) = (d_bytes, u_bytes, url, timestamp)
        {
            DownloadResult {
                download: d_b.as_f64().unwrap() / 125000 as f64,
                upload: u_b.as_f64().unwrap() / 125000 as f64,
                link: x_url.clone(),
                timestamp: x_timestamp.clone(),
            }
        } else {
            panic!("Can't find");
        }
    }
}
