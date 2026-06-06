use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.

/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterName {
    pub value: String,
}

impl Default for AdapterName {
    fn default() -> Self {
        AdapterName { value: String::new() }
    }
}

impl AdapterName {
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName { value: value.trim().to_string() })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName { value: value.into() }
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

#[cfg(test)]
mod tests {
    use super::AdapterName;

    #[test]
    fn test_adapter_name_new() {
        let name = AdapterName::new("ruff").unwrap();
        assert_eq!(name.value, "ruff");

        // Test trimming
        let name = AdapterName::new("  ruff  ").unwrap();
        assert_eq!(name.value, "ruff");

        // Test that internal spaces are preserved
        let name = AdapterName::new("my adapter").unwrap();
        assert_eq!(name.value, "my adapter");
    }

    #[test]
    fn test_adapter_name_invalid() {
        assert!(AdapterName::new("").is_err());
        assert!(AdapterName::new("   ").is_err());
        assert!(AdapterName::new("\t\n  ").is_err());
    }
}
