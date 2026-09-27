// AES403: Block 3 (inherent impl) declared before Block 2 (protocol impl).
// The 3-block contract is Block 1 type -> Block 2 protocol -> Block 3 helpers.
use crate::contract::some_protocol::SomeProtocol;

pub struct BlockOrderProcessor;

impl BlockOrderProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl SomeProtocol for BlockOrderProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}
