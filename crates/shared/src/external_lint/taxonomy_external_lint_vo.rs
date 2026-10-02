// PURPOSE: ExternalLintContext — pre-computed scan context for external lint orchestrator
//
// Contains all data the orchestrator needs to run adapter scans without performing
// any filesystem I/O. The surface layer computes this context before delegation.

use shared_config_system::taxonomy_config_system_vo::AdapterEntry;

/// Pre-computed context for external lint scan.
/// Passed from surface layer to orchestrator to eliminate agent-layer I/O.
#[derive(Debug, Clone, Default)]
pub struct ExternalLintContext {
    /// Whether the project contains Rust source files.
    pub has_rust: bool,
    /// Whether the project contains Python source files.
    pub has_python: bool,
    /// Whether the project contains JavaScript/TypeScript source files.
    pub has_js: bool,
    /// Whether the project contains Markdown source files.
    pub has_markdown: bool,
    /// Paths to ignore from config (combined with built-in defaults).
    pub ignored_paths: Vec<String>,
    /// Violation codes to drop from the report, matched case-insensitively as
    /// a substring of the code. This carries the project's `ignored_rules:`
    /// list and is how a repository turns an individual external-tool rule off
    /// (`markdownlint::MD013`) without needing a second, tool-specific config
    /// file alongside `lint_arwaky.config.yaml`.
    pub ignored_rules: Vec<String>,
    /// Adapter configuration entries from config (name, weight, timeout, enabled).
    pub config_entries: Vec<AdapterEntry>,
}
