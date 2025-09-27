src/lib.rs

* Implement examples/list_rows.rs similar to examples/metadata.rs
* Proxy pass all other parameters from CLI args to the Client::list_rows
* Test by running the list_rows example
  * The environment variables for api_key, doc_id, table_id are already set
