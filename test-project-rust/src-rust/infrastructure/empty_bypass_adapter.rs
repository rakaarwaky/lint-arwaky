// This infrastructure file violates AES016 (dead-inheritance-bypass)
// because it defines an empty adapter structure implementing the port contract with no fields or attributes.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::dummy_port::IDummyPort;

pub struct EmptyBypassAdapter;

impl IDummyPort for EmptyBypassAdapter {
    fn execute_action(&self, input: String) -> bool {
        // empty implementation to bypass checker!
        true
    }
}
