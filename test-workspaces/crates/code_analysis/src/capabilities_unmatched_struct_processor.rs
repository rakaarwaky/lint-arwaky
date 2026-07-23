// AES037: Capability struct without matching impl — should trigger routing detection
// aes: wired-by-dispatch
use crate::contract::some_protocol::SomeProtocol;

pub struct UnmatchedProcessor;

impl SomeProtocol for UnmatchedProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

pub struct OrphanStruct;
