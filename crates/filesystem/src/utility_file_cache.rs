// FR-007: File Cache — Utility layer (stateless helpers only)
// Bounded cache state lives in capabilities_file_cache.
// This module provides stateless string-keyed cache read.

use dashmap::DashMap;
use std::sync::LazyLock;

// ═══════════════════════════════════════════════════════════════
// String-keyed Cache
// ═══════════════════════════════════════════════════════════════

static STRING_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// Get cached file content by string path.
pub fn cache_get_by_str(path: &str) -> Option<String> {
    STRING_CACHE.get(path).map(|r| r.value().clone())
}
