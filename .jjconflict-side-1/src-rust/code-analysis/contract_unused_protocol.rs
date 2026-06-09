//! Protocol for detecting unused code in the project.
//!
//! Provides the interface for finding unused imports within
//! source files to help eliminate dead code.

/* UNKNOWN: SymbolName */
use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IUnusedProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName>;
}
