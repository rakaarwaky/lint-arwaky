// PURPOSE: Path normalization utilities for external tool execution (clippy, ruff, eslint, ...).
// Plain free functions — no protocol / dependency injection.
use shared_common::taxonomy_path_vo::FilePath;

/// Return `path` unchanged. External lint tools already receive absolute/normalized paths.
pub fn normalize_path(path: FilePath) -> FilePath {
    path
}

/// Resolve a capability/module `path` relative to an optional `context_path`.
/// Default behavior: the path is returned unchanged.
pub fn resolve_capabilities_path(path: FilePath, _context_path: Option<FilePath>) -> FilePath {
    path
}

/// Build a `FilePath` from a string, falling back to `fallback` when the
/// string fails `FilePath::new` validation.
pub fn resolve_or_fallback(raw: &str, fallback: FilePath) -> FilePath {
    FilePath::new(raw).unwrap_or(fallback)
}

/// Combined: parse `raw` into `FilePath` (with `fallback` on failure),
/// then resolve it relative to `context_path`.
/// Collapses the two-step call pattern repeated across 7 adapters.
pub fn resolve_or_fallback_with_context(
    raw: &str,
    fallback: FilePath,
    context_path: Option<FilePath>,
) -> FilePath {
    let fp = resolve_or_fallback(raw, fallback);
    resolve_capabilities_path(fp, context_path)
}
