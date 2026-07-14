use clap::Parser;
use coda_api::{Client, ClientGetDocDataError, DocId};
use errgonomic::{exit_result, handle};
use std::io;
use std::io::Write;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser)]
pub struct DocDataCli {
    /// API key used when talking to Coda.
    #[arg(short, long, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Identifier of the doc whose data should be downloaded.
    #[arg(short, long, env = "CODA_DOC_ID")]
    pub doc_id: DocId,
}

impl DocDataCli {
    pub async fn run(&self) -> Result<(), DocDataCliRunError> {
        use DocDataCliRunError::*;
        let client = handle!(Client::new_with_key(&self.api_key), NewWithKeyFailed);
        let doc_data = handle!(client.get_doc_data(&self.doc_id).await, GetDocDataFailed);
        let mut stdout = io::stdout().lock();
        handle!(serde_json::to_writer_pretty(&mut stdout, &doc_data), ToWriterPrettyFailed);
        handle!(writeln!(stdout), WriteNewlineFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DocDataCliRunError {
    #[error("failed to create Coda client")]
    NewWithKeyFailed { source: reqwest::Error },
    #[error("failed to get doc data")]
    GetDocDataFailed { source: Box<ClientGetDocDataError> },
    #[error("failed to write doc data")]
    ToWriterPrettyFailed { source: serde_json::Error },
    #[error("failed to write a newline")]
    WriteNewlineFailed { source: io::Error },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = DocDataCli::parse();
    exit_result(cli.run().await)
}
