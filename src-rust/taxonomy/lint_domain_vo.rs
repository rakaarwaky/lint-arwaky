use serde::{Serialize, Deserialize};

use crate::taxonomy::{ColumnNumber, FilePath, LineNumber};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), kind: "function".to_string(), file: None, start_line: None, end_line: None }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().map_or(false, |l| l.value > 0) &&
        self.end_line.as_ref().map_or(false, |l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind, self.name, file)
        } else if !self.kind.is_empty() {
            write!(f, "{} {}", self.kind, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: String,
}

impl Location {
    pub fn new() -> Self {
        Self { file: None, line: None, column: None, description: String::new() }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file { parts.push(file.value.clone()); }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 { s = format!("{}:{}", line.value, col.value); }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() { "unknown".to_string() } else { parts.join(":") };
        if self.description.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl Default for LocationList {
    fn default() -> Self {
        LocationList { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) { self.values.push(item); }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target { &self.values }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: String,
    #[serde(default)]
    pub min_value: String,
    #[serde(default)]
    pub max_value: String,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self { rule: rule.into(), min_value: String::new(), max_value: String::new() }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<String>,
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.args.join(" "))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
