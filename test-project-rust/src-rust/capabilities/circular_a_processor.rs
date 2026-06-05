// AES020 — circular-import-violation
// This file imports from circular_b_processor, which in turn imports back here.
// This creates a circular dependency between the two capability modules.

use crate::capabilities::circular_b_processor::CircularBProcessor;
use crate::taxonomy::removal_types::RemovalType;

pub struct CircularAProcessor {
    pub name: String,
}

impl CircularAProcessor {
    pub fn process_a(&self) -> String {
        let b = CircularBProcessor {
            name: "b".to_string(),
        };
        format!("A calls B: {}", b.process_b())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
