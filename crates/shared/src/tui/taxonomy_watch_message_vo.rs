// PURPOSE: WatchMessage — value object representing a watch update message in TUI taxonomy
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WatchMessage {
    pub value: String,
}

impl WatchMessage {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
