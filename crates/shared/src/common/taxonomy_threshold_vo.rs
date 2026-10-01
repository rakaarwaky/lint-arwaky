// PURPOSE: Threshold — value object for CI compliance threshold percentage
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Threshold {
    pub value: u32,
}

impl Threshold {
    /// Infallible constructor retained for compatibility. Business entry points
    /// must call `try_new` (and `collect_ci` validates defensively).
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Construct a validated CI percentage.
    pub fn try_new(value: u32) -> Result<Self, String> {
        if value <= 100 {
            Ok(Self { value })
        } else {
            Err(format!("threshold must be between 0 and 100 (got {value})"))
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl From<u32> for Threshold {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for Threshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self { value: 100 }
    }
}
