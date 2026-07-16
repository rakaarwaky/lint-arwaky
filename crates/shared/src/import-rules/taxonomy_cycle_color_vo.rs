// PURPOSE: ColorVO — DFS 3-color cycle detection state (AES205)
use serde::{Deserialize, Serialize};

/// DFS color for cycle detection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum Color {
    #[default]
    White, // unvisited
    Gray,  // currently in stack
    Black, // fully explored
}
