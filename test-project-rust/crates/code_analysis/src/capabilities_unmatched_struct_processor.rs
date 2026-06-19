// AES037: Capability struct without matching impl — should trigger routing detection
// aes: wired-by-dispatch
use crate::contract::wrong_name_port::WrongNamePort;

pub struct UnmatchedProcessor;

impl WrongNamePort for UnmatchedProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

pub struct OrphanStruct;
