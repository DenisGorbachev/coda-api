use crate::types::{Column, Table};
use crate::Client;
use std::collections::HashMap;

pub type TableId = String;

impl Client {
    pub const BASE_URL: &'static str = "https://coda.io/apis/v1";

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
        let mut all_tables = Vec::new();

        let mut tables_page_token: Option<String> = None;
        loop {
            let tables_result = self
                .list_tables(
                    &doc_id,
                    None, // limit
                    tables_page_token.as_deref(),
                    None, // sort_by
                    None, // table_types
                )
                .await;

            let tables_list = match tables_result {
                Ok(response) => response.into_inner(),
                Err(_) => break,
            };

            // Get full table details for each table reference
            for table_ref in tables_list.items {
                if let Ok(table_response) = self.get_table(&doc_id, &table_ref.id, None).await {
                    all_tables.push(table_response.into_inner());
                }
            }

            // Check if there are more pages
            if let Some(next_token) = tables_list.next_page_token {
                tables_page_token = Some(next_token.into());
            } else {
                break;
            }
        }

        all_tables
    }

    pub async fn columns_map(&self, table_ids: impl IntoIterator<Item = TableId>) -> HashMap<TableId, Vec<Column>> {
        let mut columns_map = HashMap::new();

        // First, get all docs to find the table's doc_id
        let mut docs_page_token: Option<String> = None;
        let mut doc_table_map = HashMap::new();

        loop {
            let docs_result = self
                .list_docs(
                    None, // folder_id
                    None, // in_gallery
                    None, // is_owner
                    None, // is_published
                    None, // is_starred
                    None, // limit
                    docs_page_token.as_deref(),
                    None, // query
                    None, // source_doc
                    None, // workspace_id
                )
                .await;

            let docs = match docs_result {
                Ok(response) => response.into_inner(),
                Err(_) => break,
            };

            // For each doc, get all tables to build doc_id -> table_id mapping
            for doc in docs.items {
                let mut tables_page_token: Option<String> = None;
                loop {
                    let tables_result = self
                        .list_tables(
                            &doc.id,
                            None, // limit
                            tables_page_token.as_deref(),
                            None, // sort_by
                            None, // table_types
                        )
                        .await;

                    let tables_list = match tables_result {
                        Ok(response) => response.into_inner(),
                        Err(_) => break,
                    };

                    for table_ref in tables_list.items {
                        doc_table_map.insert(table_ref.id.clone(), doc.id.clone());
                    }

                    // Check if there are more pages
                    if let Some(next_token) = tables_list.next_page_token {
                        tables_page_token = Some(next_token.into());
                    } else {
                        break;
                    }
                }
            }

            // Check if there are more docs pages
            if let Some(next_token) = docs.next_page_token {
                docs_page_token = Some(next_token.into());
            } else {
                break;
            }
        }

        // Now get columns for each requested table
        for table_id in table_ids {
            if let Some(doc_id) = doc_table_map.get(&table_id) {
                let mut columns = Vec::new();
                let mut page_token: Option<String> = None;

                loop {
                    let columns_result = self
                        .list_columns(
                            doc_id,
                            &table_id,
                            None, // limit
                            page_token.as_deref(),
                            None, // visible_only
                        )
                        .await;

                    let columns_list = match columns_result {
                        Ok(response) => response.into_inner(),
                        Err(_) => break,
                    };

                    columns.extend(columns_list.items);

                    // Check if there are more pages
                    if let Some(next_token) = columns_list.next_page_token {
                        page_token = Some(next_token.into());
                    } else {
                        break;
                    }
                }

                columns_map.insert(table_id, columns);
            }
        }

        columns_map
    }
}
