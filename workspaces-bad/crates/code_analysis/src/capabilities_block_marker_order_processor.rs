// AES403: block markers out of order — Block 3 is declared before Block 2.
// The sequence is fixed: Block 1 (struct) -> Block 2 (protocol impl) ->
// Block 3 (constructors, std traits, helpers). Reading 1 -> 3 -> 2 means the
// banner map lies about the order, so `check_block_markers` reports the
// sequence as out of order.
use crate::contract::some_protocol::SomeProtocol;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct BlockMarkerOrderProcessor;

// ─── Block 3: Module Helpers ────────────

impl BlockMarkerOrderProcessor {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Module Contract ────────────────

impl SomeProtocol for BlockMarkerOrderProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}
