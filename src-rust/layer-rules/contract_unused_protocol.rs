//! Protocol for detecting unused code in the project.
//!
//! Provides the interface for finding unused imports within
//! source files to help eliminate dead code.

use crate::source_parsing::taxonomy_path_vo::FilePath;
/* UNKNOWN: SymbolName */ use crate::naming_rules::taxonomy_symbol_vo::SymbolName;

pub trait IUnusedProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName>;
}
