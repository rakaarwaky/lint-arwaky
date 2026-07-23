// PURPOSE: DependencyEdge — Value Object representing a directed dependency edge between layers
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}
