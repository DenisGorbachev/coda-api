use coda_api::Client;
use serde_json::json;
use std::env::{args, var};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = var("CODA_API_KEY").expect("CODA_API_KEY should be set");
    let mut args = args();
    let _bin = args.next();
    let doc_id = args.next().expect("doc_id should be the first argument");
    let client = Client::new_with_key(&key)?;
    let tables = client.tables(&doc_id).await?;
    let tables_ids = tables.iter().map(|t| t.id.clone());
    // let columns_map = client.columns_map(&doc_id, tables_ids).await?;
    let output = json!({
        "tables": tables,
        // "columns_map": columns_map
    });
    println!("{output}");
    Ok(())
}
