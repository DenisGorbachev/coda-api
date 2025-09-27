use clap::{Parser, builder::BoolishValueParser};
use coda_api::{
    Client,
    types::{RowsSortBy, ValueFormat},
};
use serde_json::json;
use std::num::NonZeroU64;

fn parse_non_zero_u64(input: &str) -> Result<NonZeroU64, String> {
    let value = input.parse::<u64>().map_err(|error| format!("{error}"))?;
    NonZeroU64::new(value).ok_or_else(|| "value must be non-zero".to_owned())
}

#[derive(Clone, Debug, Parser)]
pub struct ListRowsCli {
    /// API key used when talking to Coda.
    #[arg(long, short, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Identifier of the doc whose table should be queried.
    #[arg(long, short = 'd', env = "CODA_DOC_ID")]
    pub doc_id: String,

    /// Identifier or name of the table to list rows from.
    #[arg(long, short = 't', env = "CODA_TABLE_ID")]
    pub table_id: String,

    /// Maximum number of rows to fetch in a single page.
    #[arg(long, value_parser = parse_non_zero_u64, value_name = "COUNT")]
    pub limit: Option<NonZeroU64>,

    /// Opaque token indicating the next page to fetch.
    #[arg(long, value_name = "TOKEN")]
    pub page_token: Option<String>,

    /// Filter expression in the form <column>:<value>.
    #[arg(long, value_name = "EXPRESSION")]
    pub query: Option<String>,

    /// Sort ordering applied to the returned rows.
    #[arg(long, value_name = "ORDER")]
    pub sort_by: Option<RowsSortBy>,

    /// Sync token returned from an earlier request to fetch incremental updates.
    #[arg(long, value_name = "TOKEN")]
    pub sync_token: Option<String>,

    /// Whether the API should return column names instead of IDs.
    #[arg(long, value_parser = BoolishValueParser::new(), value_name = "BOOL")]
    pub use_column_names: Option<bool>,

    /// Desired representation of cell values.
    #[arg(long, value_name = "FORMAT")]
    pub value_format: Option<ValueFormat>,

    /// When true, limits the response to visible rows and columns only.
    #[arg(long, value_parser = BoolishValueParser::new(), value_name = "BOOL")]
    pub visible_only: Option<bool>,
}

impl ListRowsCli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new_with_key(&self.api_key)?;
        let rows = client
            .list_rows(&self.doc_id, &self.table_id, self.limit, self.page_token.as_deref(), self.query.as_deref(), self.sort_by, self.sync_token.as_deref(), self.use_column_names, self.value_format, self.visible_only)
            .await?
            .into_inner();
        let output = json!({ "rows": rows });
        println!("{output}");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ListRowsCli::parse();
    cli.run().await
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    ListRowsCli::command().debug_assert();
}
