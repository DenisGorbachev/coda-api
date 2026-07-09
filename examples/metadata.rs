use clap::Parser;
use coda_api::{Client, ClientGetMetadataError};
use errgonomic::{exit_result, handle};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Clone, Debug, Parser)]
pub struct MetadataCli {
    /// API key used when talking to Coda.
    #[arg(long, short, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Identifier of the doc whose metadata should be queried.
    #[arg(long, short = 'd', env = "CODA_DOC_ID")]
    pub doc_id: String,
}

impl MetadataCli {
    pub async fn run(&self) -> Result<(), MetadataCliRunError> {
        use MetadataCliRunError::*;
        let client = handle!(Client::new_with_key(&self.api_key), ClientNewWithKeyFailed);
        let metadata = handle!(client.get_metadata(&self.doc_id).await, ClientGetMetadataFailed);
        let mut stdout = io::stdout().lock();
        handle!(serde_json::to_writer_pretty(&mut stdout, &metadata), WriteMetadataFailed);
        handle!(writeln!(stdout), WriteNewlineFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum MetadataCliRunError {
    #[error("failed to create Coda client")]
    ClientNewWithKeyFailed { source: reqwest::Error },
    #[error("failed to get metadata")]
    ClientGetMetadataFailed { source: Box<ClientGetMetadataError> },
    #[error("failed to write metadata")]
    WriteMetadataFailed { source: serde_json::Error },
    #[error("failed to write newline")]
    WriteNewlineFailed { source: io::Error },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = MetadataCli::parse();
    exit_result(cli.run().await)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    MetadataCli::command().debug_assert();
}
