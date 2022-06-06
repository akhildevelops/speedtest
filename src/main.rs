use speedtest::*;
use std::env;
fn main() {
    let airtable_b_id = env::var(defaults::AIRTABLE_BASE_ID).unwrap();
    let airtable_a_k = env::var(defaults::AIRTABLE_AUTH_KEY).unwrap();
    let client = Ookla::new();
    let result = client.test();
    let airtable_client = AirtableClient::new(airtable_a_k, airtable_b_id);
    airtable_client.table("1".into()).insert_row(result);
}
