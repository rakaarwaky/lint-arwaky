// PURPOSE: utility_orphan_regex_patterns — regex patterns for graph resolution
use regex::Regex;
use std::sync::OnceLock;

/// Bug 8 fix: Allow digits in module name (e.g., `mod foo2;`, `mod v2_api;`)
pub fn pub_mod_path_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)"#).ok()
    })
    .as_ref()
}

/// Bug 6 fix: Handle `pub(crate) mod foo;` visibility qualifiers
pub fn plain_mod_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok()
    })
    .as_ref()
}

/// Bug 4 fix: Add word boundary \b to prevent matching in comments/strings/literals
pub fn import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\b(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*(?:\{[^}]*\})?)").ok()
    })
    .as_ref()
}

/// Regex for Python relative imports: `from . import X` or `from .module import X`
pub fn python_relative_import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?s)from\s+(\.{1,3})\s+import\s*\(([^)]+)\)|from\s+(\.{1,3})\s+import\s+([^\n]+)",
        )
        .ok()
    })
    .as_ref()
}

/// Bug 12 fix: Only run for Python files (class inheritance pattern)
pub fn inh_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
}

/// TypeScript/JavaScript import patterns:
/// - `import { X } from './path'` (named)
/// - `import X from './path'` (default)
/// - `import * as X from './path'` (namespace)
/// - `import './path'` (side-effect)
/// - `import Default, { Named } from './path'` (default + named)
pub fn ts_import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"\bimport\s+(?:\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|(\w+)\s*,\s*\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|(\w+)\s+from\s*['"]([^'"]+)['"]|\*\s*as\s+(\w+)\s+from\s*['"]([^'"]+)['"]|(['"]([^'"]+)['"]))"#).ok()
    })
    .as_ref()
}

/// TypeScript/JavaScript export re-exports:
/// - `export { X } from './path'` (named)
/// - `export * from './path'` (re-export all)
/// - `export { X } from './path'` (side-effect, no from)
pub fn ts_export_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        // Match: export { X, Y } from './path', export * from './path'
        Regex::new(
            r#"\bexport\s+(?:\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|\*\s*from\s*['"]([^'"]+)['"])?"#,
        )
        .ok()
    })
    .as_ref()
}

/// Bug 1 fix: Match multi-segment `pub use crate::common::taxonomy_action_vo;`
/// No lookahead (regex crate doesn't support it) — filter at code level
pub fn pub_use_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
            // Match `pub use crate::module::submodule;` multi-segment paths
            Regex::new(r"pub\s+use\s+(?:crate|super|self)::([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)\s*;").ok()
        })
    .as_ref()
}

/// Bug 2 fix: No negative lookahead (regex crate doesn't support it)
/// Filter crate::/super::/self:: prefixes at code level in Pass 5b
pub fn pub_use_relative_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"pub\s+use\s+([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)\s*;").ok()
    })
    .as_ref()
}
