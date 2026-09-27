// PURPOSE: ToolName — newtype for external tool executable names
use serde::{Deserialize, Serialize};

/// A validated external tool identifier (e.g. `"clippy"`, `"ruff"`, `"eslint"`).
///
/// The newtype prevents passing a tool name where a different domain value
/// (a file path, a lint code, a layer name) is expected — the compiler
/// rejects the mismatch before the code runs.
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
