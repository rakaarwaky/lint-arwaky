// PURPOSE: ColorVO — DFS 3-color cycle detection state (AES205)
use serde::{Deserialize, Serialize};

/// DFS color for cycle detection.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Color {
    White, // unvisited
    Gray,  // currently in stack
    Black, // fully explored
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}
