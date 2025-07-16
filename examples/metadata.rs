use coda_api::Client;
use serde_json::json;
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("CODA_API_KEY").expect("CODA_API_KEY should be set");
    let client = Client::new_with_key(&key).unwrap();
    let tables = client.tables().await;
    let tables_ids = tables.iter().map(|t| t.id.clone());
    let columns_map = client.columns_map(tables_ids).await;
    let output = json!({"tables": tables, "columns_map": columns_map});
    println!("{output}")
}
