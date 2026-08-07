// PURPOSE: Orphan-specific analysis VOs + re-exports of graph types from filesystem.
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use crate::common::taxonomy_lint_result_vo::LintResultList;

// ── Re-export graph types from filesystem (canonical home) ──
pub use crate::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext;
pub use crate::filesystem::taxonomy_filesystem_vo::ImportGraph;
pub use crate::filesystem::taxonomy_filesystem_vo::InboundLinkMap;
pub use crate::filesystem::taxonomy_filesystem_vo::InheritanceMap;

use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}
