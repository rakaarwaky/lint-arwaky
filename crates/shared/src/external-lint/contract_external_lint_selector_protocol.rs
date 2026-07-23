// PURPOSE: IExternalLintSelectorProtocol — protocol for selecting adapters based on detected languages
use crate::common::taxonomy_adapter_list_vo::AdapterNameList;
use async_trait::async_trait;

/// Protocol for choosing which external-lint adapters to run.
///
/// Based on booleans indicating the presence of Rust, Python, or TypeScript
/// files in the project, the selector returns the list of adapter names
/// that should be invoked during the external linting phase.
#[async_trait]
pub trait IExternalLintSelectorProtocol: Send + Sync {
    fn select_adapters(&self, has_rs: bool, has_py: bool, has_js: bool) -> AdapterNameList;
}
