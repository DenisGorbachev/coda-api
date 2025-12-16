use crate::types::{Column, ColumnList, Doc, DocList, GetTableResponse, ListTablesResponse, NextPageToken, Row, RowList, TableList, TableReference};
use crate::{Error, RawClient, types};
use progenitor_client::{ClientHooks, ClientInfo, OperationInfo, ResponseValue, encode_path};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[cfg(feature = "time")]
mod duration_value_parser;
mod impl_from_for_value;
mod items_list;
mod parse_cell_value;
mod parse_rich_value;
mod rich_rows;
mod row;
mod string_or_f64;
mod value_format_provider;

#[cfg(feature = "time")]
pub use duration_value_parser::*;
pub use items_list::*;
pub use parse_cell_value::*;
pub use parse_rich_value::*;
pub use rich_rows::*;
pub(crate) use string_or_f64::*;
pub use value_format_provider::*;

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

impl<T> PaginatedResponse<T> for ItemsList<T> {
    fn items(&self) -> &Vec<T> {
        &self.items
    }

    fn next_page_token(&self) -> Option<&NextPageToken> {
        self.next_page_token.as_ref()
    }

    fn into_items(self) -> Vec<T> {
        self.items
    }
}

impl RawClient {
    pub const BASE_URL: &'static str = "https://coda.io/apis/v1";

    pub fn new_with_key(api_key: &str) -> reqwest::Result<Self> {
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

    #[allow(clippy::too_many_arguments)]
    pub async fn list_rows_correct<'a, T: DeserializeOwned + ValueFormatProvider>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, query: Option<&'a str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&'a str>, use_column_names: Option<bool>, visible_only: Option<bool>) -> Result<ResponseValue<ItemsList<T>>, Error<types::ListRowsResponse>> {
        let url = format!("{}/docs/{}/tables/{}/rows", self.baseurl, encode_path(doc_id), encode_path(table_id_or_name),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(::reqwest::header::HeaderName::from_static("api-version"), ::reqwest::header::HeaderValue::from_static(Self::api_version()));
        let value_format = Some(T::value_format());
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(::reqwest::header::ACCEPT, ::reqwest::header::HeaderValue::from_static("application/json"))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("pageToken", &page_token))
            .query(&progenitor_client::QueryParam::new("query", &query))
            .query(&progenitor_client::QueryParam::new("sortBy", &sort_by))
            .query(&progenitor_client::QueryParam::new("syncToken", &sync_token))
            .query(&progenitor_client::QueryParam::new("useColumnNames", &use_column_names))
            .query(&progenitor_client::QueryParam::new("valueFormat", &value_format))
            .query(&progenitor_client::QueryParam::new("visibleOnly", &visible_only))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_rows_rich",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            429u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a row
    ///
    ///Returns details about a row in a table.
    ///
    ///Sends a `GET` request to
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
    /// - `use_column_names`: Use column names instead of column IDs in the
    ///   returned output. This is generally discouraged as it is fragile. If
    ///   columns are renamed, code using original names may throw errors.
    ///
    /// - `value_format`: The format that cell values are returned as.
    pub async fn get_row_correct<'a, T: DeserializeOwned + ValueFormatProvider>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, use_column_names: Option<bool>) -> Result<ResponseValue<T>, Error<types::GetRowResponse>> {
        let url = format!("{}/docs/{}/tables/{}/rows/{}", self.baseurl, encode_path(doc_id), encode_path(table_id_or_name), encode_path(row_id_or_name),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(::reqwest::header::HeaderName::from_static("api-version"), ::reqwest::header::HeaderValue::from_static(Self::api_version()));
        let value_format = Some(T::value_format());
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(::reqwest::header::ACCEPT, ::reqwest::header::HeaderValue::from_static("application/json"))
            .query(&progenitor_client::QueryParam::new("useColumnNames", &use_column_names))
            .query(&progenitor_client::QueryParam::new("valueFormat", &value_format))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_row",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            429u16 => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Insert/upsert rows
    ///
    ///Inserts rows into a table, optionally updating existing rows if any
    /// upsert key columns are provided. This endpoint will always return a 202,
    /// so long as the doc and table exist and are accessible (and the update is
    /// structurally valid). Row inserts/upserts are generally processed within
    /// several seconds. Note: this endpoint only works for base tables, not
    /// views. When upserting, if multiple rows match the specified key
    /// column(s), they will all be updated with the specified value.
    ///
    ///
    ///Sends a `POST` request to `/docs/{docId}/tables/{tableIdOrName}/rows`
    ///
    ///Arguments:
    /// - `doc_id`: ID of the doc.
    /// - `table_id_or_name`: ID or name of the table. Names are discouraged
    ///   because they're easily prone to being changed by users. If you're
    ///   using a name, be sure to URI-encode it.
    /// - `disable_parsing`: If true, the API will not attempt to parse the data
    ///   in any way.
    /// - `body`: Rows to insert or upsert.
    pub async fn upsert_rows_correct<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowsUpsert) -> Result<ResponseValue<RowsUpsertResultCorrect>, Error<types::UpsertRowsResponse>> {
        let url = format!("{}/docs/{}/tables/{}/rows", self.baseurl, encode_path(doc_id), encode_path(table_id_or_name),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(::reqwest::header::HeaderName::from_static("api-version"), ::reqwest::header::HeaderValue::from_static(Self::api_version()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(::reqwest::header::ACCEPT, ::reqwest::header::HeaderValue::from_static("application/json"))
            .json(&body)
            .query(&progenitor_client::QueryParam::new("disableParsing", &disable_parsing))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upsert_rows",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
        let url = format!("{}/docs/{}/tables/{}/rows/{}", self.baseurl, encode_path(doc_id), encode_path(table_id_or_name), encode_path(row_id_or_name),);
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

#[derive(Debug, Error)]
pub enum ClientTablesError {
    #[error("list tables request failed: {source}")]
    ListTablesFailed {
        #[source]
        source: Error<ListTablesResponse>,
    },
    #[error("get table request failed: {source}")]
    GetTableFailed {
        #[source]
        source: Error<GetTableResponse>,
    },
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
    pub id: RowId,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

///`RowsUpsertResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The result of a rows insert/upsert operation.",
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/DocumentMutateResponse"
///    },
///    {
///      "type": "object",
///      "properties": {
///        "addedRowIds": {
///          "description": "Row IDs for rows that will be added. Only
/// applicable when keyColumns is not set or empty.",
///          "examples": [
///            [
///              "i-bCdeFgh",
///              "i-CdEfgHi"
///            ]
///          ],
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ],
///  "x-schema-name": "RowsUpsertResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct RowsUpsertResultCorrect {
    #[serde(rename = "addedRowIds", default)]
    pub added_row_ids: Vec<String>,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

pub fn format_row_url(doc_id: &str, table_id: &str, row_id: &str) -> String {
    format!("https://coda.io/d/_d{doc_id}#_tu{table_id}/_ru{row_id}")
}

pub async fn paginate_all<T, R, F, Fut, E>(mut request_fn: F) -> Result<Vec<T>, E>
where
    // TODO: Remove the `T: Clone` requirement
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
            Err(err) => return Err(err),
        }
    }

    Ok(all_items)
}
