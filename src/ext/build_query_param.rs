use serde_json::Value;

pub fn build_query_param(column: &str, value: &str) -> String {
    let column = Value::String(column.to_owned());
    let value = Value::String(value.to_owned());
    format!("{column}:{value}")
}
