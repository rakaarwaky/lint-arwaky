use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportInfo {
    pub line: LineNumber,
    pub module: String,
    #[serde(default)]
    pub name: Option<String>,
}

impl ImportInfo {
    pub fn new(line: LineNumber, module: String) -> Self {
        Self { line, module, name: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveViolation {
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub type_name: String,
}

impl PrimitiveViolation {
    pub fn new(line: LineNumber, column: ColumnNumber, type_name: String) -> Self {
        Self { line, column, type_name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportInfoList {
    #[serde(default)]
    pub values: Vec<ImportInfo>,
}

impl ImportInfoList {
    pub fn new() -> Self { Self { values: Vec::new() } }
    pub fn push(&mut self, item: ImportInfo) { self.values.push(item); }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveViolationList {
    #[serde(default)]
    pub values: Vec<PrimitiveViolation>,
}

impl PrimitiveViolationList {
    pub fn new() -> Self { Self { values: Vec::new() } }
    pub fn push(&mut self, item: PrimitiveViolation) { self.values.push(item); }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
}
