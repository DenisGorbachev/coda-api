use crate::{DocMetadata, RichRow, TableId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DocData {
    #[serde(flatten)]
    pub metadata: DocMetadata,
    pub rows: BTreeMap<TableId, Vec<RichRow>>,
}
