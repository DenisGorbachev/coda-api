use crate::types::{Column, ColumnList, Doc, DocList, ListTablesResponse, NextPageToken, Row, RowList, Table, TableList, TableReference};
use crate::{types, Client, Error};
use std::collections::HashMap;

pub type TableId = String;

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
        Fut: std::future::Future<Output = Result<R, E>>,
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

    pub async fn tables(&self, doc_id: &str) -> Result<Vec<Table>, Error<ListTablesResponse>> {
        // Use the generic pagination helper to get all table references
        let table_refs = self
            .paginate_all(move |page_token| async move {
                self.list_tables(doc_id, None, page_token.as_deref(), None, None)
                    .await
                    .map(|response| response.into_inner())
            })
            .await?;

        // Get full table details for each table reference
        let mut all_tables = Vec::new();
        for table_ref in table_refs {
            match self.get_table(doc_id, &table_ref.id, None).await {
                Ok(table_response) => all_tables.push(table_response.into_inner()),
                Err(_) => continue, // Skip tables that fail to load details
            }
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

    pub async fn rows(&self, doc_id: &str, table_id: &str, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<Vec<Row>, Error<types::ListRowsResponse>> {
        // Use the generic pagination helper to get all rows
        self.paginate_all(move |page_token| async move {
            self.list_rows(doc_id, table_id, None, page_token.as_deref(), None, None, sync_token, use_column_names, value_format, None)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    pub async fn rows_map(&self, doc_id: &str, table_ids: impl IntoIterator<Item = TableId>, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<HashMap<TableId, Vec<Row>>, Error<types::ListRowsResponse>> {
        let rows_futures = table_ids.into_iter().map(|table_id| async {
            let rows = self
                .rows(doc_id, &table_id, sync_token, use_column_names, value_format)
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
}
