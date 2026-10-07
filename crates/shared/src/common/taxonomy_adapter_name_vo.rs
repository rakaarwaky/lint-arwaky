// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use crate::list_wrapper_vo;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// ─── Merged from taxonomy_adapter_list_vo ─────────────────────

list_wrapper_vo!(AdapterNameList, AdapterName);

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

// ─── Merged from taxonomy_tool_name_vo ────────────────────────

/// ToolName — validated external tool executable name (e.g. `"clippy"`,
/// `"ruff"`, `"eslint"`). The newtype prevents passing a tool name where a
/// different domain value (a file path, a lint code, a layer name) is
/// expected — the compiler rejects the mismatch before the code runs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolName(String);

impl ToolName {
    /// Create a `ToolName` from anything string-like.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the underlying string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ToolName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for ToolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
