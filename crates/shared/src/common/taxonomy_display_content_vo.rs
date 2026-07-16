// PURPOSE: DisplayContent — value object for formatted display output (previews, human-readable sizes, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayContent {
    pub value: String,
}

impl DisplayContent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn contains(&self, pattern: &str) -> bool {
        self.value.contains(pattern)
    }
}

impl From<String> for DisplayContent {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DisplayContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
