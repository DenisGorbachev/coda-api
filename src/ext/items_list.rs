use crate::types::{NextPageLink, NextPageToken, NextSyncToken};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ItemsList<T> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub items: Vec<T>,
    #[serde(rename = "nextPageLink", default, skip_serializing_if = "Option::is_none")]
    pub next_page_link: Option<NextPageLink>,
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<NextPageToken>,
    #[serde(rename = "nextSyncToken", default, skip_serializing_if = "Option::is_none")]
    pub next_sync_token: Option<NextSyncToken>,
}
