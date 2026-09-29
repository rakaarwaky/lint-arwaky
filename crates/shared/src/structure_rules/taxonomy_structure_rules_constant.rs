// PURPOSE: structure invariant rule codes, violation types, and folder vocabulary for AES701–AES703

/// ─── AES701 — Shared folder rules ───────────────────────────────────────────
/// `shared/` is the kernel: taxonomy, utility, and contract files only.
/// A doc pair (FRD+BACKLOG) in shared fires this rule.
pub const RULE_CODE_SHARED_PURITY: &str = "AES701";

/// A shared folder holds a file whose layer does not belong there.
pub const SHARED_PURITY_VIOLATION_FORBIDDEN_FILES: &str = "shared_has_forbidden_files";

/// A shared (kernel) folder carries a doc pair; it must be empty of docs.
pub const SHARED_PURITY_VIOLATION_HAS_DOCS: &str = "shared_has_docs";

/// ─── AES702 — Feature folder rules ──────────────────────────────────────────
/// A feature folder holds capabilities + agent orchestrator, carries no
/// foreign layer files (utility, surface, taxonomy, contract), and
/// documents itself with FRD.md + BACKLOG.md beside its source.
pub const RULE_CODE_FEATURE_HEALTH: &str = "AES702";

/// A feature folder holds capabilities but no agent orchestrator.
pub const FEATURE_HEALTH_VIOLATION_MISSING_AGENT: &str = "feature_missing_agent";

/// A feature folder holds an agent orchestrator but no capability.
pub const FEATURE_HEALTH_VIOLATION_MISSING_CAPABILITY: &str = "feature_missing_capability";

/// A feature folder holds a file from a foreign layer (utility, surface, taxonomy, contract).
pub const FEATURE_HEALTH_VIOLATION_FORBIDDEN_FILES: &str = "feature_has_forbidden_files";

/// A feature folder lacks its FRD.md + BACKLOG.md doc pair.
pub const FEATURE_HEALTH_VIOLATION_NO_DOC_PAIR: &str = "feature_missing_doc_pair";

/// A folder carries a doc pair but holds no orchestrator; reverse-direction check.
pub const FEATURE_HEALTH_VIOLATION_REVERSE_MISSING_ORCHESTRATOR: &str =
    "doc_pair_without_orchestrator";

/// ─── AES703 — Surface folder rules ─────────────────────────────────────────
/// A surface-dominated folder carries surface files only (plus permitted
/// utility/root/barrel support) and records its shape in DESIGN.md.
pub const RULE_CODE_SURFACE_PURITY: &str = "AES703";

/// A surface folder holds a capabilities or agent file that belongs in a feature folder.
pub const SURFACE_PURITY_VIOLATION_MISPLACED_FILES: &str = "surface_has_misplaced_files";

/// A surface-dominated folder carries no DESIGN.md.
pub const SURFACE_PURITY_VIOLATION_NO_DESIGN: &str = "surface_missing_design_md";

/// Doc filenames referenced by AES702, so structure-rules can check their
/// presence without importing the doc-rules constants.
pub const FRD_DOC: &str = "FRD.md";
pub const BACKLOG_DOC: &str = "BACKLOG.md";

/// ─── Shape constants ────────────────────────────────────────────────────────
/// The shared folder name, locked across every language member.
pub const KERNEL_DIR: &str = "shared";

/// The workspace member directories that hold feature folders.
pub const MEMBER_DIRS: &[&str] = &["crates", "modules", "packages"];

/// Layer prefixes a shared folder rejects.
pub const SHARED_FORBIDDEN_PREFIXES: &[&str] = &["capabilities_", "agent_", "surface_"];

/// Layer prefixes forbidden in a feature folder: they belong in shared or a surface folder.
pub const FEATURE_FORBIDDEN_PREFIXES: &[&str] = &["utility_", "surface_", "taxonomy_", "contract_"];

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

/// The document file names that mark a folder's purpose and content.
pub const FEATURE_DOC_PAIR: &[&str] = &["FRD.md", "BACKLOG.md"];
pub const SURFACE_DOC: &str = "DESIGN.md";
