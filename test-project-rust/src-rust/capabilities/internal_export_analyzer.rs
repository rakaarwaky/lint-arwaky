// AES013 — internal-all-forbidden
// This non-barrel capability file declares a `pub use` re-export list
// which is forbidden — only mod.rs barrel files should define public API boundaries.

use crate::taxonomy::removal_types::RemovalType;

// Forbidden: pub re-exports in a non-barrel file (AES013)
pub use crate::taxonomy::removal_types::RemovalRequest;
pub use crate::taxonomy::removal_types::RemovalResult;

pub struct InternalExportAnalyzer {
    pub name: String,
}

impl InternalExportAnalyzer {
    pub fn analyze(&self) -> RemovalType {
        RemovalType::Background
    }
}
