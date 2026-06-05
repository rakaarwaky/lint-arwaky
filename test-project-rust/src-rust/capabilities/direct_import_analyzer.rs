// This capability file violates AES007 (contract-barrel-violation)
// because it imports directly from contract::dummy_port instead of the contract barrel (contract).
use crate::contract::dummy_port::IDummyPort;
use crate::taxonomy::removal_types::RemovalType;

pub struct DirectImportAnalyzer {
    pub port: Option<Box<dyn IDummyPort>>,
}

impl DirectImportAnalyzer {
    pub fn new(port: Box<dyn IDummyPort>) -> Self {
        Self { port: Some(port) }
    }
}
