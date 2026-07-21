// PURPOSE: Percentage — value object for percentage values (0.0–100.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Percentage {
    pub value: f64,
}

impl Percentage {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Percentage {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}%", self.value)
    }
}

impl Default for Percentage {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}
