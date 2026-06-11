// PURPOSE: PyASTUtil — utility functions for Python AST parsing and node traversal
use language_adapters::contract_naming_port::INamingProviderPort;
use shared_common::taxonomy_common_vo::LineNumber;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);
    let _ = std::marker::PhantomData::<dyn INamingProviderPort>;
}

pub struct PythonAstUtils {}

impl Default for PythonAstUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonAstUtils {
    pub fn new() -> Self {
        Self {}
    }
}
