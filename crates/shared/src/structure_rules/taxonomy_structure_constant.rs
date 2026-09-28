// PURPOSE: structure invariant rule codes, violation types, and folder vocabulary for AES701–AES703

/// ─── AES701 — Shared folder purity ──────────────────────────────────────────
pub const RULE_CODE_SHARED_PURITY: &str = "AES701";

/// A shared folder holds a file whose layer does not belong there.
pub const SHARED_PURITY_VIOLATION_FORBIDDEN_FILES: &str = "shared_has_forbidden_files";

/// ─── AES702 — Feature folder health ─────────────────────────────────────────
pub const RULE_CODE_FEATURE_HEALTH: &str = "AES702";

/// A feature folder holds capabilities but no agent orchestrator.
pub const FEATURE_HEALTH_VIOLATION_MISSING_AGENT: &str = "feature_missing_agent";

/// A feature folder holds an agent orchestrator but no capability.
pub const FEATURE_HEALTH_VIOLATION_MISSING_CAPABILITY: &str = "feature_missing_capability";

/// ─── AES703 — Surface folder purity ─────────────────────────────────────────
pub const RULE_CODE_SURFACE_PURITY: &str = "AES703";

/// A surface folder holds a capabilities or agent file that belongs in a feature folder.
pub const SURFACE_PURITY_VIOLATION_MISPLACED_FILES: &str = "surface_has_misplaced_files";

/// ─── Shape constants ────────────────────────────────────────────────────────
/// The shared folder name, locked across every language member.
pub const KERNEL_DIR: &str = "shared";

/// The workspace member directories that hold feature folders.
pub const MEMBER_DIRS: &[&str] = &["crates", "modules", "packages"];

/// Layer prefixes a shared folder accepts.
pub const SHARED_ALLOWED_PREFIXES: &[&str] = &["taxonomy_", "utility_", "contract_"];

/// Layer prefixes a shared folder rejects.
pub const SHARED_FORBIDDEN_PREFIXES: &[&str] = &["capabilities_", "agent_", "surface_"];

/// The agent orchestrator suffix that marks a feature folder.
pub const ORCHESTRATOR_SUFFIX: &str = "_orchestrator";

/// The capabilities layer prefix.
pub const CAPABILITIES_PREFIX: &str = "capabilities_";

/// The agent layer prefix.
pub const AGENT_PREFIX: &str = "agent_";

/// The surface layer prefix.
pub const SURFACE_PREFIX: &str = "surface_";

/// The root layer prefix, accepted alongside utilities in a surface folder.
pub const ROOT_PREFIX: &str = "root_";

/// Files a surface folder may keep: barrels, entry wiring, packaging, and
/// per-layer support files that the surface itself consumes directly.
pub const SURFACE_ALLOWED_FILENAMES: &[&str] = &[
    "lib.rs",
    "mod.rs",
    "__init__.py",
    "index.ts",
    "index.js",
    "Cargo.toml",
    "pyproject.toml",
    "package.json",
];

/// Directories a structure audit never descends into.
pub const SKIPPED_DIRS: &[&str] = &["benches", "tests", "target", "node_modules", "__pycache__"];
