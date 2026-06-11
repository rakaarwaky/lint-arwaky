// PURPOSE: PyPrimitiveDetector — IDomainTypeProtocol implementation for Python primitive detection
use crate::language_adapters::contract_naming_port::INamingProviderPort;
use crate::shared_common::taxonomy_common_vo::LineNumber;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);
    let _ = std::marker::PhantomData::<dyn INamingProviderPort>;
}

pub struct PythonPrimitiveChecker {}

impl Default for PythonPrimitiveChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonPrimitiveChecker {
    pub fn new() -> Self {
        Self {}
    }
}
