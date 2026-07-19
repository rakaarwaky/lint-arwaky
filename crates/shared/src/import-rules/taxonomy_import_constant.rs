// PURPOSE: taxonomy_import_constant — compile-time constants for import-rules layer
// All domain values MUST be named constants. No hardcoded literals in layer files.

/// Known derive-macro imports that Rust compiler consumes implicitly.
/// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
/// attributes, so they must never be flagged as unused.
pub const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

/// Layer prefixes used for filename-based layer detection.
pub const LAYER_PREFIXES: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("infrastructure_", "infrastructure"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];

/// Rust entry file names that should be skipped during scope-level checks.
pub const RUST_ENTRY_FILES: &[&str] = &["mod.rs", "lib.rs", "main.rs"];

/// Python entry file names that should be skipped during mandatory checks.
pub const PYTHON_ENTRY_FILES: &[&str] = &["__init__.py"];

/// Source code file extensions for file collection.
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];
