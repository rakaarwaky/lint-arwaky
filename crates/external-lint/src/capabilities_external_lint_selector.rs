use shared::common::taxonomy_adapter_list_vo::{AdapterName, AdapterNameList};
use shared::common::taxonomy_common_vo::bool;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// PURPOSE: CapabilitiesExternalLintSelector — selects adapters based on detected languages
//
// Pure business logic: maps language flags to adapter name lists.
// No I/O, no external dependencies.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesExternalLintSelector {
    rust_adapters: Vec<AdapterName>,
    python_adapters: Vec<AdapterName>,
    js_adapters: Vec<AdapterName>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IExternalLintSelectorProtocol for CapabilitiesExternalLintSelector {
    fn select_adapters(&self, has_rs: bool, has_py: bool, has_js: bool) -> AdapterNameList {
        let mut adapter_names = Vec::new();
        if has_rs.value() {
            for name in self.rust_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_py.value() {
            for name in self.python_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_js.value() {
            for name in self.js_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        AdapterNameList::new(adapter_names)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CapabilitiesExternalLintSelector {
    pub fn new(
        rust_adapters: Vec<AdapterName>,
        python_adapters: Vec<AdapterName>,
        js_adapters: Vec<AdapterName>,
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
                AdapterName::raw("clippy"),
                AdapterName::raw("rustfmt"),
                AdapterName::raw("cargo-audit"),
            ],
            vec![
                AdapterName::raw("ruff"),
                AdapterName::raw("mypy"),
                AdapterName::raw("bandit"),
            ],
            vec![
                AdapterName::raw("eslint"),
                AdapterName::raw("prettier"),
                AdapterName::raw("tsc"),
            ],
        )
    }
}
