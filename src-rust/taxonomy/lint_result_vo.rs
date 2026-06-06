use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    pub fn new(file: FilePath, line: LineNumber, column: ColumnNumber, code: ErrorCode, message: LintMessage, source: Option<AdapterName>, severity: Severity, enclosing_scope: Option<ScopeRef>, related_locations: LocationList,) -> Self {
        Self { file, line, column, code, message, source, severity, enclosing_scope, related_locations }
    }

    pub fn position(&self) -> Position {
        Position { line: self.line.clone(), column: self.column.clone() }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!("{}:{}:{}:{:?}", self.file, self.line, self.code, self.source))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResultList {
    pub values: Vec<LintResult>,
}

impl LintResultList {
    pub fn new(value: Vec<LintResult>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LintResult> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LintResult) {
        self.values.push(item);
    }
    pub fn append(&mut self, item: LintResult) {
        self.values.push(item);
    }
}
