// AES403: a block marker above Block 3.
// The sequence is Block 1 (struct) -> Block 2 (protocol impl) -> Block 3
// (constructors, std traits, helpers). A `Block 4:` means the file has outgrown
// the shape its reader is promised, so `check_block_markers` reports the marker
// beyond Block 3.
use crate::contract::some_protocol::SomeProtocol;

// ─── Block 1: Struct Definition ────────────────────────────

pub struct ExtraBlockMarkerProcessor;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl SomeProtocol for ExtraBlockMarkerProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl ExtraBlockMarkerProcessor {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 4: Trailing Helpers ────────────────────────────

impl ExtraBlockMarkerProcessor {
    fn normalize(&self, raw: &str) -> String {
        raw.trim().to_lowercase()
    }
}
