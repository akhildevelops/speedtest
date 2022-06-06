use crate::defaults;
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
pub struct AirtableClient {
    reqwest_client: Client,
    bearer_token: String,
    base_id: String,
}

pub struct AirtableTable<'a> {
    table_id: String,
    client: &'a AirtableClient,
}

#[derive(Serialize)]
struct AirtableJson<T>
where
    T: Serialize,
{
    records: [Records<T>; 1],
}

#[derive(Serialize, Debug)]
struct Records<T>
where
    T: Serialize,
{
    fields: T,
}

impl<'a> AirtableTable<'a> {
    pub fn insert_row<T>(&'a mut self, row: T) -> &'a Self
    where
        T: Serialize + Debug + Sized,
    {
        let url = format!(
            "{}/{}/Table {}",
            defaults::AIRTABLE_URL,
            self.client.base_id,
            self.table_id
        );
        let records = AirtableJson {
            records: [Records { fields: row }],
        };
        println!(
            "{}, {}",
            serde_json::to_string_pretty(&records).unwrap(),
            url
        );
        let response = self
            .client
            .reqwest_client
            .post(url)
            .bearer_auth(&self.client.bearer_token)
            .json(&records)
            .send();
        println!("{:?}", response.unwrap().json::<Value>());

        self
    }
}

impl AirtableClient {
    pub fn new(bearer_token: String, base_id: String) -> Self {
        let client = Client::new();
        Self {
            reqwest_client: client,
            bearer_token,
            base_id,
        }
    }

    pub fn table(&self, table_id: String) -> AirtableTable {
        AirtableTable {
            table_id,
            client: &self,
        }
    }
}
