// AES020 — circular-import-violation
// This file imports from circular_a_processor, which imports back here.
// Circular imports create initialization order problems and break DI wiring.

use crate::capabilities::circular_a_processor::CircularAProcessor;
use crate::taxonomy::removal_types::RemovalType;

pub struct CircularBProcessor {
    pub name: String,
}

impl CircularBProcessor {
    pub fn process_b(&self) -> String {
        let a = CircularAProcessor {
            name: "a".to_string(),
        };
        format!("B calls A: {}", a.get_name())
    }
}
