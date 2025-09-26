use clap::Parser;
use coda_api::Client;
use serde_json::json;

#[derive(Clone, Debug, Parser)]
pub struct MetadataCli {
    #[arg(long, short, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    #[arg(long, short, env = "CODA_DOC_ID")]
    pub doc_id: String,
}

impl MetadataCli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new_with_key(&self.api_key)?;
        let tables = client.tables(&self.doc_id).await?;
        let output = json!({
            "tables": tables,
        });
        println!("{output}");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = MetadataCli::parse();
    cli.run().await
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    MetadataCli::command().debug_assert();
}
