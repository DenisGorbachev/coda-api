use clap::Parser;
use coda_api::{Client, ClientTablesError, Error, TableId, types};
use errgonomic::handle;
use serde::Serialize;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Clone, Debug, Parser)]
pub struct MetadataCli {
    /// API key used when talking to Coda.
    #[arg(long, short, env = "CODA_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Identifier of the doc whose metadata should be queried.
    #[arg(long, short = 'd', env = "CODA_DOC_ID")]
    pub doc_id: String,

    /// Include ACL sharing metadata and ACL settings.
    #[arg(long)]
    pub include_acl: bool,

    /// Include explicit doc permissions.
    #[arg(long)]
    pub include_permissions: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct Metadata {
    pub doc: types::Doc,
    pub pages: Vec<types::Page>,
    pub tables: Vec<types::Table>,
    pub columns: BTreeMap<TableId, Vec<types::Column>>,
    pub formulas: Vec<types::FormulaReference>,
    pub controls: Vec<types::ControlReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<types::AclMetadata>,
    #[serde(rename = "aclSettings", skip_serializing_if = "Option::is_none")]
    pub acl_settings: Option<types::AclSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<types::Permission>>,
}

impl Metadata {
    pub async fn get(client: &Client, doc_id: &str, include_acl: bool, include_permissions: bool) -> Result<Self, MetadataGetError> {
        use MetadataGetError::*;
        let doc = handle!(client.get_doc(doc_id).await, GetDocFailed).into_inner();
        let pages = handle!(all_pages(client, doc_id).await, AllPagesFailed);
        let tables = handle!(client.tables(doc_id).await, TablesFailed);
        let table_ids = tables.iter().map(|table| table.id.clone());
        let columns = handle!(client.columns_map(doc_id, table_ids).await, ColumnsMapFailed)
            .into_iter()
            .collect();
        let formulas = handle!(all_formula_refs(client, doc_id).await, AllFormulaRefsFailed);
        let controls = handle!(all_control_refs(client, doc_id).await, AllControlRefsFailed);
        let (sharing, acl_settings) = if include_acl {
            let sharing = handle!(client.get_sharing_metadata(doc_id).await, GetSharingMetadataFailed).into_inner();
            let acl_settings = handle!(client.get_acl_settings(doc_id).await, GetAclSettingsFailed).into_inner();
            (Some(sharing), Some(acl_settings))
        } else {
            (None, None)
        };
        let permissions = if include_permissions {
            Some(handle!(all_permissions(client, doc_id).await, AllPermissionsFailed))
        } else {
            None
        };
        Ok(Self {
            doc,
            pages,
            tables,
            columns,
            formulas,
            controls,
            sharing,
            acl_settings,
            permissions,
        })
    }
}

impl MetadataCli {
    pub async fn run(&self) -> Result<(), MetadataCliRunError> {
        use MetadataCliRunError::*;
        let client = handle!(Client::new_with_key(&self.api_key), ClientNewWithKeyFailed);
        let metadata = handle!(Metadata::get(&client, &self.doc_id, self.include_acl, self.include_permissions).await, MetadataGetFailed);
        let output = handle!(serde_json::to_string(&metadata), SerializeMetadataFailed);
        println!("{output}");
        Ok(())
    }
}

macro_rules! define_all_items {
    ($function:ident, $item:ty, $error:ident, $variant:ident, |$client:ident, $doc_id:ident, $page_token:ident| $request:expr) => {
        async fn $function($client: &Client, $doc_id: &str) -> Result<Vec<$item>, $error> {
            use $error::*;
            let mut $page_token: Option<String> = None;
            let mut items = Vec::new();
            loop {
                let page = handle!(($request).await, $variant).into_inner();
                items.extend(page.items);
                $page_token = page.next_page_token.map(Into::into);
                if $page_token.is_none() {
                    return Ok(items);
                }
            }
        }
    };
}

define_all_items!(all_pages, types::Page, AllPagesError, ListPagesFailed, |client, doc_id, page_token| client.list_pages(doc_id, None, page_token.as_deref()));

define_all_items!(all_formula_refs, types::FormulaReference, AllFormulaRefsError, ListFormulasFailed, |client, doc_id, page_token| client.list_formulas(doc_id, None, page_token.as_deref(), None));

define_all_items!(all_control_refs, types::ControlReference, AllControlRefsError, ListControlsFailed, |client, doc_id, page_token| client.list_controls(doc_id, None, page_token.as_deref(), None));

define_all_items!(all_permissions, types::Permission, AllPermissionsError, GetPermissionsFailed, |client, doc_id, page_token| client.get_permissions(doc_id, None, page_token.as_deref()));

#[derive(Error, Debug)]
pub enum MetadataCliRunError {
    #[error("failed to create Coda client")]
    ClientNewWithKeyFailed { source: reqwest::Error },
    #[error("failed to get metadata")]
    MetadataGetFailed { source: Box<MetadataGetError> },
    #[error("failed to serialize metadata")]
    SerializeMetadataFailed { source: serde_json::Error },
}

#[derive(Error, Debug)]
pub enum MetadataGetError {
    #[error("failed to get doc")]
    GetDocFailed { source: Error<types::GetDocResponse> },
    #[error("failed to list all pages")]
    AllPagesFailed { source: AllPagesError },
    #[error("failed to list tables")]
    TablesFailed { source: ClientTablesError },
    #[error("failed to list columns")]
    ColumnsMapFailed { source: Error<types::ListColumnsResponse> },
    #[error("failed to list all formula references")]
    AllFormulaRefsFailed { source: AllFormulaRefsError },
    #[error("failed to list all control references")]
    AllControlRefsFailed { source: AllControlRefsError },
    #[error("failed to get sharing metadata")]
    GetSharingMetadataFailed { source: Error<types::GetSharingMetadataResponse> },
    #[error("failed to get ACL settings")]
    GetAclSettingsFailed { source: Error<types::GetAclSettingsResponse> },
    #[error("failed to list all permissions")]
    AllPermissionsFailed { source: AllPermissionsError },
}

#[derive(Error, Debug)]
pub enum AllPagesError {
    #[error("failed to list pages")]
    ListPagesFailed { source: Error<types::ListPagesResponse> },
}

#[derive(Error, Debug)]
pub enum AllFormulaRefsError {
    #[error("failed to list formulas")]
    ListFormulasFailed { source: Error<types::ListFormulasResponse> },
}

#[derive(Error, Debug)]
pub enum AllControlRefsError {
    #[error("failed to list controls")]
    ListControlsFailed { source: Error<types::ListControlsResponse> },
}

#[derive(Error, Debug)]
pub enum AllPermissionsError {
    #[error("failed to get permissions")]
    GetPermissionsFailed { source: Error<types::GetPermissionsResponse> },
}

#[tokio::main]
async fn main() -> Result<(), MetadataCliRunError> {
    let cli = MetadataCli::parse();
    cli.run().await
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    MetadataCli::command().debug_assert();
}
