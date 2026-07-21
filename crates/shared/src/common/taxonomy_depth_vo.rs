// PURPOSE: DepthCount — value object for directory scan depth
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DepthCount {
    pub value: usize,
}

impl DepthCount {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for DepthCount {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DepthCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
