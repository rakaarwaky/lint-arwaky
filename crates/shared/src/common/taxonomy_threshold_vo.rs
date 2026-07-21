// PURPOSE: Threshold — value object for CI compliance threshold percentage
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Threshold {
    pub value: u32,
}

impl Threshold {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl From<u32> for Threshold {
    fn from(value: u32) -> Self {
        Self { value }
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
