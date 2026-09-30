// PURPOSE: CacheKey — newtype for config-cache lookup keys
use serde::{Deserialize, Serialize};

// A cache key used for parsing configs (no longer uses IConfigCacheProtocol trait).
/// Newtype wrapping `String` so the contract does not accidentally mix a
/// cache key with a YAML blob or a file path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CacheKey(String);

impl CacheKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CacheKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CacheKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
