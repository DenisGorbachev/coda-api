use crate::types::{Column, ColumnList, Doc, DocList, GetTableResponse, ListTablesResponse, NextPageToken, Row, RowList, Table, TableList, TableReference};
use crate::{Client, Error, types};
use derive_more::Error;
use error_handling::handle;
use fmt_derive::Display;
use progenitor_client::{ClientInfo, ResponseValue, encode_path};
use std::collections::HashMap;

pub type DocId = String;

pub type TableId = String;

pub type RowId = String;

// Generic pagination trait for Coda API responses
pub trait PaginatedResponse<T> {
    fn items(&self) -> &Vec<T>;
    fn next_page_token(&self) -> Option<&NextPageToken>;
    fn into_items(self) -> Vec<T>;
}

// Generic pagination helper
pub struct PaginationState {
    pub next_page_token: Option<String>,
}

impl PaginationState {
    pub fn new() -> Self {
        Self {
            next_page_token: None,
        }
    }

    pub fn update_from_response<T, R: PaginatedResponse<T>>(&mut self, response: &R) {
        self.next_page_token = response.next_page_token().map(|token| token.clone().into());
    }

    pub fn has_more_pages(&self) -> bool {
        self.next_page_token.is_some()
    }

    pub fn page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        Self::new()
    }
}

// Implement PaginatedResponse for the specific types we need
impl PaginatedResponse<TableReference> for TableList {
    fn items(&self) -> &Vec<TableReference> {
        &self.items
    }

    fn next_page_token(&self) -> Option<&NextPageToken> {
        self.next_page_token.as_ref()
    }

    fn into_items(self) -> Vec<TableReference> {
        self.items
    }
}

impl PaginatedResponse<Column> for ColumnList {
    fn items(&self) -> &Vec<Column> {
        &self.items
    }

    fn next_page_token(&self) -> Option<&NextPageToken> {
        self.next_page_token.as_ref()
    }

    fn into_items(self) -> Vec<Column> {
        self.items
    }
}

impl PaginatedResponse<Doc> for DocList {
    fn items(&self) -> &Vec<Doc> {
        &self.items
    }

    fn next_page_token(&self) -> Option<&NextPageToken> {
        self.next_page_token.as_ref()
    }

    fn into_items(self) -> Vec<Doc> {
        self.items
    }
}

impl PaginatedResponse<Row> for RowList {
    fn items(&self) -> &Vec<Row> {
        &self.items
    }

    fn next_page_token(&self) -> Option<&NextPageToken> {
        self.next_page_token.as_ref()
    }

    fn into_items(self) -> Vec<Row> {
        self.items
    }
}

impl Client {
    pub const BASE_URL: &'static str = "https://coda.io/apis/v1";

    // Generic pagination helper that collects all pages into a Vec
    pub async fn paginate_all<T, R, F, Fut, E>(&self, mut request_fn: F) -> Result<Vec<T>, E>
    where
        T: Clone,
        R: PaginatedResponse<T>,
        F: FnMut(Option<String>) -> Fut,
        Fut: Future<Output = Result<R, E>>,
    {
        let mut all_items = Vec::new();
        let mut pagination_state = PaginationState::new();

        loop {
            match request_fn(pagination_state.next_page_token.clone()).await {
                Ok(response) => {
                    all_items.extend(response.items().iter().cloned());

                    if let Some(next_token) = response.next_page_token() {
                        pagination_state.next_page_token = Some(next_token.clone().into());
                    } else {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(all_items)
    }

    pub fn new_with_key(api_key: &str) -> reqwest::Result<Client> {
        let authorization_header = format!("Bearer {api_key}")
            .parse()
            .expect("API key should be valid");

        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(reqwest::header::AUTHORIZATION, authorization_header);

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        let client = Self::new_with_client(Self::BASE_URL, client_with_custom_defaults);

        Ok(client)
    }

    pub async fn table_refs(&self, doc_id: &str) -> Result<Vec<TableReference>, Error<ListTablesResponse>> {
        self.paginate_all(move |page_token| async move {
            self.list_tables(doc_id, None, page_token.as_deref(), None, None)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    pub async fn tables(&self, doc_id: &str) -> Result<Vec<Table>, ClientTablesError> {
        use ClientTablesError::*;
        // Use the generic pagination helper to get all table references
        let table_refs = handle!(self.table_refs(doc_id).await, ListTablesFailed);

        // Get full table details for each table reference
        let mut all_tables = Vec::new();
        for table_ref in table_refs {
            let table_response = handle!(self.get_table(doc_id, &table_ref.id, None).await, GetTableFailed);
            all_tables.push(table_response.into_inner());
        }

        Ok(all_tables)
    }

    pub async fn columns_map(&self, doc_id: &str, table_ids: impl IntoIterator<Item = TableId>) -> Result<HashMap<TableId, Vec<Column>>, Error<types::ListColumnsResponse>> {
        let mut columns_map = HashMap::new();

        // Now get columns for each requested table
        for table_id in table_ids {
            let table_id_ref = table_id.as_ref();
            let columns = self
                .paginate_all(move |page_token| async move {
                    self.list_columns(doc_id, table_id_ref, None, page_token.as_deref(), None)
                        .await
                        .map(|response| response.into_inner())
                })
                .await?;

            columns_map.insert(table_id, columns);
        }

        Ok(columns_map)
    }

    pub async fn rows(&self, doc_id: &str, table_id: &str, query: Option<&str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<Vec<Row>, Error<types::ListRowsResponse>> {
        // Use the generic pagination helper to get all rows
        self.paginate_all(move |page_token| async move {
            self.list_rows(doc_id, table_id, None, page_token.as_deref(), query, sort_by, sync_token, use_column_names, value_format, None)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    pub async fn rows_map(&self, doc_id: &str, table_ids: impl IntoIterator<Item = TableId>, query: Option<&str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<HashMap<TableId, Vec<Row>>, Error<types::ListRowsResponse>> {
        let rows_futures = table_ids.into_iter().map(|table_id| async {
            let rows = self
                .rows(doc_id, &table_id, query, sort_by, sync_token, use_column_names, value_format)
                .await?;
            Ok::<(TableId, Vec<Row>), Error<types::ListRowsResponse>>((table_id, rows))
        });

        let mut rows_map = HashMap::new();

        for future in rows_futures {
            let (table_id, rows) = future.await?;
            rows_map.insert(table_id, rows);
        }

        Ok(rows_map)
    }

    ///Update row
    ///
    ///Updates the specified row in the table. This endpoint will always return
    /// a 202, so long as the row exists and is accessible (and the update is
    /// structurally valid). Row updates are generally processed within several
    /// seconds. When updating using a name as opposed to an ID, an arbitrary
    /// row will be affected.
    ///
    ///
    ///Sends a `PUT` request to
    /// `/docs/{docId}/tables/{tableIdOrName}/rows/{rowIdOrName}`
    ///
    ///Arguments:
    /// - `doc_id`: ID of the doc.
    /// - `table_id_or_name`: ID or name of the table. Names are discouraged
    ///   because they're easily prone to being changed by users. If you're
    ///   using a name, be sure to URI-encode it.
    /// - `row_id_or_name`: ID or name of the row. Names are discouraged because
    ///   they're easily prone to being changed by users. If you're using a
    ///   name, be sure to URI-encode it. If there are multiple rows with the
    ///   same value in the identifying column, an arbitrary one will be
    ///   selected.
    ///
    /// - `disable_parsing`: If true, the API will not attempt to parse the data
    ///   in any way.
    /// - `body`: Row update.
    pub async fn update_row_correct<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowUpdate) -> Result<ResponseValue<RowUpdateResultCorrect>, Error<types::UpdateRowResponse>> {
        let url = format!("{}/docs/{}/tables/{}/rows/{}", self.baseurl, encode_path(&doc_id.to_string()), encode_path(&table_id_or_name.to_string()), encode_path(&row_id_or_name.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(::reqwest::header::HeaderName::from_static("api-version"), ::reqwest::header::HeaderValue::from_static(Self::api_version()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(::reqwest::header::ACCEPT, ::reqwest::header::HeaderValue::from_static("application/json"))
            .json(&body)
            .query(&progenitor_client::QueryParam::new("disableParsing", &disable_parsing))
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            429u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[derive(Error, Display, Debug)]
pub enum ClientTablesError {
    ListTablesFailed { source: Error<ListTablesResponse> },
    GetTableFailed { source: Error<GetTableResponse> },
}

///`RowUpdateResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The result of a row update.",
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/DocumentMutateResponse"
///    },
///    {
///      "type": "object",
///      "required": [
///        "id"
///      ],
///      "properties": {
///        "id": {
///          "description": "ID of the updated row.",
///          "examples": [
///            "i-tuVwxYz"
///          ],
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ],
///  "x-schema-name": "RowUpdateResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct RowUpdateResultCorrect {
    #[serde(rename = "id")]
    id: RowId,
    #[serde(rename = "requestId")]
    pub request_id: String,
}
// impl ::std::convert::From<&Self> for crate::types::RowUpdateResult {
//     fn from(value: &crate::types::RowUpdateResult) -> Self {
//         value.clone()
//     }
// }
