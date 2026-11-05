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
    /// Explicit Markdown file list for the markdownlint adapter. When set, the
    /// adapter hands these files to the CLI instead of a directory glob, so it
    /// never walks ignored or vendored trees. `None` keeps the glob fallback.
    /// Computed by the surface layer (zero I/O at the agent layer).
    pub markdown_files: Option<Vec<String>>,
    /// Scan timeout the surface layer resolved for the markdownlint adapter:
    /// the `AdapterEntry.timeout` from config when present, else the
    /// adapter's built-in default. Other adapters keep their own constants.
    pub markdown_timeout: f64,
}

impl ExternalLintContext {
    /// Timeout the surface layer resolved for one adapter: the config entry's
    /// `timeout` when the adapter is listed, else the caller-supplied
    /// built-in default. Zero I/O — the entries are pre-computed.
    pub fn adapter_timeout(&self, adapter: &str, default: f64) -> f64 {
        self.config_entries
            .iter()
            .find(|e| e.name.value() == adapter)
            .map(|e| e.timeout)
            .unwrap_or(default)
    }
}

/// Timeout the surface layer resolved for the markdownlint adapter:
/// `AdapterEntry.timeout` from config when present, else the adapter's
/// built-in default.
pub fn markdown_scan_timeout(context: &ExternalLintContext) -> f64 {
    context.adapter_timeout(
        "markdownlint",
        crate::taxonomy_external_lint_constant::MARKDOWNLINT_DEFAULT_SCAN_TIMEOUT_SECS,
    )
}

/// True when the project's `ignored_rules` silence every markdownlint rule
/// (the orchestrator's substring filter is case-insensitive, so a bare `MD`
/// pattern matches every `markdownlint::MD###` code, and a
/// `markdownlint::` prefix silences the whole adapter family). The adapter
/// uses this to skip spawning the subprocess entirely.
///
/// Cross-adapter note: the bare `MD` branch is safe because the orchestrator
///'s post-scan rule filter (case-insensitive substring match, applied to every
/// adapter's findings) will drop any surviving finding whose code contains
/// `MD` — so no other adapter can leak a finding through once a bare `MD`
/// entry is present. If that filter semantics ever changes, re-audit this
/// function.
pub fn markdown_fully_suppressed(context: &ExternalLintContext) -> bool {
    let prefix = crate::taxonomy_external_lint_constant::MARKDOWNLINT_FULL_SUPPRESSION_PREFIX;
    context.ignored_rules.iter().any(|p| {
        let up = p.to_uppercase();
        up == "MD" || up.starts_with(prefix)
    })
}
