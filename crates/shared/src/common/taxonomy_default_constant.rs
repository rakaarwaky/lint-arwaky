// PURPOSE: Default constants — single source of truth for ignore patterns and exceptions
// Used by: filesystem (build_file_index), config_system (ignored_paths), rule crates (exceptions)

/// Default directories to always ignore during filesystem scanning.
/// These are universal across all languages and project types.
/// Config-specified ignored_paths are merged ON TOP of these defaults.
pub const DEFAULT_IGNORED_PATHS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
    ".venv",
    "__pycache__",
    ".mypy_cache",
    ".ruff_cache",
    "tests",
    "benches",
];

/// Default file exceptions for rule checkers (naming, import, quality, orphan).
/// These files are barrel/entry-point files that should NOT be checked by rules.
/// Config exceptions are merged ON TOP of these defaults.
///
/// Note: main.rs, main.py, main.ts are NOT included — they should get violations
/// so they can be renamed to root_*_entry format per AES naming convention.
pub const DEFAULT_RULE_EXCEPTIONS: &[&str] = &[
    "lib.rs",
    "mod.rs",
    "build.rs",
    "__init__.py",
    "index.js",
    "index.ts",
    "index.tsx",
];
