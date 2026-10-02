// AES403: no block markers at all.
// A capability declares Block 1 (struct) -> Block 2 (protocol impl) ->
// Block 3 (constructors, std traits, helpers). Each block needs its banner so
// the reader is given a map of the file. This one declares none, so
// `check_block_markers` reports "declares no block markers".
use crate::contract::some_protocol::SomeProtocol;

pub struct NoBlockMarkerProcessor;

impl SomeProtocol for NoBlockMarkerProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

impl NoBlockMarkerProcessor {
    pub fn new() -> Self {
        Self
    }
}
