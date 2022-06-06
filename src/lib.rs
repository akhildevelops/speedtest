<<<<<<< HEAD
mod airtable;
mod client;
pub mod defaults;
pub use airtable::AirtableClient;
pub use client::Ookla;

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use std::env;
    #[test]
    fn test_speedtest() {
        let client = Ookla::new();
        let result = client.test();
        println!("{:?}", result);
    }

    #[test]
    fn test_integrate() {
        let client = Ookla::new();
        let result = client.test();
        let airtable_b_id = env::var("AIRTABLE_BASE_ID").unwrap();
        let airtable_a_k = env::var("AIRTABLE_AUTH_KEY").unwrap();
        let airtable_client = AirtableClient::new(airtable_a_k, airtable_b_id);
        airtable_client.table("1".into()).insert_row(result);
    }

    #[test]
    fn test_airtable() {
        let airtable_b_id = env::var("AIRTABLE_BASE_ID").unwrap();
        let airtable_a_k = env::var("AIRTABLE_AUTH_KEY").unwrap();
        println!("{},{}", airtable_a_k, airtable_b_id);
        let val = json!({
            "records": [
                {
                    "fields": {
                        "Date": "2022-06-01T17:47:29Z",
                        "DownloadSpeed": 152.25 as f32,
                        "UploadSpeed": 157.65 as f32
                    }
                }
            ]
        });
        let airtable_client = AirtableClient::new(airtable_a_k, airtable_b_id);
        airtable_client.table("1".into()).insert_row(&val);
    }
}
=======
mod default;
mod speedtest;
mod utils;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ping_test() {
//         let local_client = ;
//         let server_url = "http://speedtest.sec.rcil.gov.in:8080/speedtest/latency.txt?x=14234";
//         local_client.ping(server_url);
//     }
// }
>>>>>>> 038b7764f98187a427c3a088b5a091461fb22ac4
