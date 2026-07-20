// PURPOSE: Path normalization utilities for external tool execution (clippy, ruff, eslint, ...).
// Plain free functions — no protocol / dependency injection.
use crate::common::taxonomy_path_vo::FilePath;

/// Return `path` unchanged. External lint tools already receive absolute/normalized paths.
pub fn normalize_path(path: FilePath) -> FilePath {
    path
}

/// Resolve a capability/module `path` relative to an optional `context_path`.
/// Default behavior: the path is returned unchanged.
pub fn resolve_capabilities_path(path: FilePath, _context_path: Option<FilePath>) -> FilePath {
    path
}
