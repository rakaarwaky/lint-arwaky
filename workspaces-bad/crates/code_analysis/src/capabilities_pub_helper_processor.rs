// AES403: Block 3 helper is fully `pub` — leaks capability internals.
// Use `fn` (private) or `pub(crate)` instead of `pub`.
use crate::contract::some_protocol::SomeProtocol;

pub struct PubHelperProcessor;

impl SomeProtocol for PubHelperProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

impl PubHelperProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn internal_compute(value: &str) -> usize {
        value.len()
    }
}
