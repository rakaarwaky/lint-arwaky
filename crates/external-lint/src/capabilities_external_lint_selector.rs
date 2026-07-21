use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// PURPOSE: CapabilitiesExternalLintSelector — selects adapters based on detected languages
//
// Pure business logic: maps language flags to adapter name lists.
// No I/O, no external dependencies.

use shared::common::taxonomy_common_vo::bool;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesExternalLintSelector {
    rust_adapters: Vec<String>,
    python_adapters: Vec<String>,
    js_adapters: Vec<String>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IExternalLintSelectorProtocol for CapabilitiesExternalLintSelector {
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
    ) -> Vec<String> {
        let mut adapter_names = Vec::new();
        if has_rs.value() {
            adapter_names.extend(self.rust_adapters.iter().cloned());
        }
        if has_py.value() {
            adapter_names.extend(self.python_adapters.iter().cloned());
        }
        if has_js.value() {
            adapter_names.extend(self.js_adapters.iter().cloned());
        }
        adapter_names
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CapabilitiesExternalLintSelector {
    pub fn new(
        rust_adapters: Vec<String>,
        python_adapters: Vec<String>,
        js_adapters: Vec<String>,
    ) -> Self {
        Self {
            rust_adapters,
            python_adapters,
            js_adapters,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            vec![
                "clippy".to_string(),
                "rustfmt".to_string(),
                "cargo-audit".to_string(),
            ],
            vec!["ruff".to_string(), "mypy".to_string(), "bandit".to_string()],
            vec![
                "eslint".to_string(),
                "prettier".to_string(),
                "tsc".to_string(),
            ],
        )
    }
}

