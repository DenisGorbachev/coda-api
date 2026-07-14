use clap::Parser;
use coda_api::{Client, ClientGetDocMetadataError, DocId};
use errgonomic::{exit_result, handle};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser)]
pub struct DocMetadataCli {
    /// API key used when talking to Coda.
    #[arg(long, short, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Identifier of the doc whose metadata should be queried.
    #[arg(long, short = 'd', env = "CODA_DOC_ID")]
    pub doc_id: DocId,
}

impl DocMetadataCli {
    pub async fn run(&self) -> Result<(), DocMetadataCliRunError> {
        use DocMetadataCliRunError::*;
        let client = handle!(Client::new_with_key(&self.api_key), NewWithKeyFailed);
        let metadata = handle!(client.get_doc_metadata(&self.doc_id).await, GetDocMetadataFailed);
        let mut stdout = io::stdout().lock();
        handle!(serde_json::to_writer_pretty(&mut stdout, &metadata), ToWriterPrettyFailed);
        handle!(writeln!(stdout), WriteNewlineFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DocMetadataCliRunError {
    #[error("failed to create Coda client")]
    NewWithKeyFailed { source: reqwest::Error },
    #[error("failed to get doc metadata")]
    GetDocMetadataFailed { source: Box<ClientGetDocMetadataError> },
    #[error("failed to write doc metadata")]
    ToWriterPrettyFailed { source: serde_json::Error },
    #[error("failed to write newline")]
    WriteNewlineFailed { source: io::Error },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = DocMetadataCli::parse();
    exit_result(cli.run().await)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    DocMetadataCli::command().debug_assert();
}
