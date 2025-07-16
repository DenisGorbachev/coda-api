use crate::types::{Column, ColumnList, Doc, DocList, NextPageToken, Table, TableList, TableReference};
use crate::Client;
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

impl Client {
    pub const BASE_URL: &'static str = "https://coda.io/apis/v1";

    // Generic pagination helper that collects all pages into a Vec
    pub async fn paginate_all<T, R, F, Fut>(&self, mut request_fn: F) -> Vec<T>
    where
        T: Clone,
        R: PaginatedResponse<T>,
        F: FnMut(Option<String>) -> Fut,
        Fut: std::future::Future<Output = Result<R, Box<dyn std::error::Error>>>,
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
                Err(_) => break,
            }
        }

        all_items
    }

    pub fn new_with_key(api_key: &str) -> reqwest::Result<Client> {
        let authorization_header = format!("Bearer {api_key}");

        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            authorization_header
                .parse()
                .expect("API key should be valid"),
        );

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        let client = Self::new_with_client(Self::BASE_URL, client_with_custom_defaults);

        Ok(client)
    }

    pub async fn tables(&self, doc_id: String) -> Vec<Table> {
        // Use the generic pagination helper to get all table references
        let doc_id_clone = doc_id.clone();
        let table_refs = self
            .paginate_all(move |page_token| {
                let doc_id = doc_id_clone.clone();
                async move {
                    self.list_tables(&doc_id, None, page_token.as_deref(), None, None)
                        .await
                        .map(|response| response.into_inner())
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                }
            })
            .await;

        // Get full table details for each table reference
        let mut all_tables = Vec::new();
        for table_ref in table_refs {
            if let Ok(table_response) = self.get_table(&doc_id, &table_ref.id, None).await {
                all_tables.push(table_response.into_inner());
            }
        }

        all_tables
    }

    pub async fn columns_map(&self, table_ids: impl IntoIterator<Item = TableId>) -> HashMap<TableId, Vec<Column>> {
        let mut columns_map = HashMap::new();

        // First, get all docs to find the table's doc_id
        let docs = self
            .paginate_all(move |page_token| {
                async move {
                    self.list_docs(
                        None, // folder_id
                        None, // in_gallery
                        None, // is_owner
                        None, // is_published
                        None, // is_starred
                        None, // limit
                        page_token.as_deref(),
                        None, // query
                        None, // source_doc
                        None, // workspace_id
                    )
                    .await
                    .map(|response| response.into_inner())
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                }
            })
            .await;

        // Build doc_id -> table_id mapping
        let mut doc_table_map = HashMap::new();
        for doc in docs {
            let doc_id = doc.id.clone();
            let table_refs = self
                .paginate_all(move |page_token| {
                    let doc_id = doc_id.clone();
                    async move {
                        self.list_tables(&doc_id, None, page_token.as_deref(), None, None)
                            .await
                            .map(|response| response.into_inner())
                            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                    }
                })
                .await;

            for table_ref in table_refs {
                doc_table_map.insert(table_ref.id.clone(), doc.id.clone());
            }
        }

        // Now get columns for each requested table
        for table_id in table_ids {
            if let Some(doc_id) = doc_table_map.get(&table_id) {
                let doc_id = doc_id.clone();
                let table_id_clone = table_id.clone();
                let columns = self
                    .paginate_all(move |page_token| {
                        let doc_id = doc_id.clone();
                        let table_id = table_id_clone.clone();
                        async move {
                            self.list_columns(&doc_id, &table_id, None, page_token.as_deref(), None)
                                .await
                                .map(|response| response.into_inner())
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                        }
                    })
                    .await;

                columns_map.insert(table_id, columns);
            }
        }

        columns_map
    }
}
