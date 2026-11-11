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

#[cfg(test)]
mod tests {
    use super::*;
    use shared_config_system::taxonomy_config_system_vo::{AdapterEntry, AdapterStatus};

    fn entry(name: &str, timeout: f64) -> AdapterEntry {
        let mut e = AdapterEntry::new(
            shared_common::taxonomy_adapter_name_vo::AdapterName::new(name).unwrap(),
            AdapterStatus::Enabled,
            1.0,
        );
        e.timeout = timeout;
        e
    }

    #[test]
    fn adapter_timeout_uses_config_entry_when_present() {
        let ctx = ExternalLintContext {
            config_entries: vec![entry("clippy", 15.0)],
            ..Default::default()
        };
        assert_eq!(ctx.adapter_timeout("clippy", 30.0), 15.0);
    }

    #[test]
    fn adapter_timeout_returns_default_when_absent() {
        let ctx = ExternalLintContext {
            config_entries: Vec::new(),
            ..Default::default()
        };
        assert_eq!(ctx.adapter_timeout("clippy", 30.0), 30.0);
    }
}
