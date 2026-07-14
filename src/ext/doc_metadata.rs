use crate::TableId;
use crate::types::{Column, Control, Doc, Formula, Page, Table};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DocMetadata {
    pub doc: Doc,
    pub pages: Vec<Page>,
    pub tables: Vec<Table>,
    pub columns: BTreeMap<TableId, Vec<Column>>,
    pub formulas: Vec<Formula>,
    pub controls: Vec<Control>,
}
