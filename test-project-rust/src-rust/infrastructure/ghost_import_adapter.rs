// This infrastructure file violates AES015 (unused-mandatory-import)
// because it imports taxonomy::removal_types::RemovalType but never uses it.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::dummy_port::IDummyPort;

pub struct GhostImportAdapter {
    pub active: bool,
}

impl IDummyPort for GhostImportAdapter {
    fn execute_action(&self, input: String) -> bool {
        self.active
    }
}
