// PURPOSE: taxonomy_import_rules_constant — compile-time constants for import-rules layer.
// DERIVE_MACROS removed — AST attribute parsing handles derive detection natively.

/// Layer prefixes used for filename-based layer detection.
pub const LAYER_PREFIXES: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("utility_", "utility"),
    ("capabilities_", "capabilities"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];

/// Rust entry file names that should be skipped during scope-level checks.
/// Python entry file names that should be skipped during mandatory checks.
/// Source code file extensions for file collection.
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];

/// Directories to skip during file collection.
/// Delegates to the single source of truth in `taxonomy_default_constant`.
pub const DEFAULT_SKIP_DIRS: &[&str] =
    shared_common::taxonomy_default_constant::DEFAULT_IGNORED_PATHS;

/// Rule code for AES201 — Forbidden Import
pub const AES201_RULE_CODE: &str = "AES201";

/// Rule code for AES202 — Mandatory Import
pub const AES202_RULE_CODE: &str = "AES202";
