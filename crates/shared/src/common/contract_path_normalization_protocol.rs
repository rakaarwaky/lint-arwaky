// PURPOSE: IPathNormalizationProtocol — contract for path normalization utilities.
// Abstracts path normalization so implementations can be swapped via DI.
// The default implementation uses pure utility functions from
// utility_path_normalization (free functions, no state).
use crate::common::taxonomy_path_vo::FilePath;

/// Trait that normalizes file paths and resolves capability paths.
pub trait IPathNormalizationProtocol: Send + Sync {
    /// Normalize a file path (currently returns unchanged for external tools).
    fn normalize_path(&self, path: &FilePath) -> FilePath;

    /// Resolve a capability/module path relative to an optional context path.
    fn resolve_capabilities_path(
        &self,
        path: &FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
