use crate::types::{CurrencyAmount, ImageStatus, LinkedDataType, NextPageLink, NextPageToken, NextSyncToken, RowType, ScalarValue, TableReference};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichRowList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub items: Vec<RichRow>,
    #[serde(rename = "nextPageLink", default, skip_serializing_if = "Option::is_none")]
    pub next_page_link: Option<NextPageLink>,
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<NextPageToken>,
    #[serde(rename = "nextSyncToken", default, skip_serializing_if = "Option::is_none")]
    pub next_sync_token: Option<NextSyncToken>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichRow {
    #[serde(rename = "browserLink")]
    pub browser_link: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub href: String,
    pub id: String,
    pub index: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TableReference>,
    #[serde(rename = "type")]
    pub type_: RowType,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub values: HashMap<String, RichValue>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RichValue {
    Single(RichSingleValue),
    Collection(Vec<RichValueEntry>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RichValueEntry {
    Single(RichSingleValue),
    Many(Vec<RichSingleValue>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RichSingleValue {
    Scalar(ScalarValue),
    Currency(RichCurrencyValue),
    Image(RichImageValue),
    Person(RichPersonValue),
    Url(RichUrlValue),
    Row(RichRowReference),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichCurrencyValue {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: LinkedDataType,
    #[serde(rename = "additionalType", default, skip_serializing_if = "Option::is_none")]
    pub additional_type: Option<String>,
    pub currency: String,
    pub amount: CurrencyAmount,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichImageValue {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: LinkedDataType,
    #[serde(rename = "additionalType", default, skip_serializing_if = "Option::is_none")]
    pub additional_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageStatus>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichPersonValue {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: LinkedDataType,
    #[serde(rename = "additionalType", default, skip_serializing_if = "Option::is_none")]
    pub additional_type: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichUrlValue {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: LinkedDataType,
    #[serde(rename = "additionalType", default, skip_serializing_if = "Option::is_none")]
    pub additional_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RichRowReference {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: LinkedDataType,
    #[serde(rename = "additionalType")]
    pub additional_type: String,
    pub name: String,
    pub url: String,
    #[serde(rename = "tableId")]
    pub table_id: String,
    #[serde(rename = "rowId")]
    pub row_id: String,
    #[serde(rename = "tableUrl")]
    pub table_url: String,
}
