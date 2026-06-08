//! Protocol for detecting unused code in the project.
//!
//! Provides the interface for finding unused imports within
//! source files to help eliminate dead code.

use crate::taxonomy::FilePath;
use crate::taxonomy::SymbolName;

pub trait IUnusedProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName>;
}
