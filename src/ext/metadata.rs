use crate::TableId;
use crate::types::{Column, ControlReference, Doc, FormulaReference, Page, Table};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Metadata {
    pub doc: Doc,
    pub pages: Vec<Page>,
    pub tables: Vec<Table>,
    pub columns: BTreeMap<TableId, Vec<Column>>,
    pub formulas: Vec<FormulaReference>,
    pub controls: Vec<ControlReference>,
}
