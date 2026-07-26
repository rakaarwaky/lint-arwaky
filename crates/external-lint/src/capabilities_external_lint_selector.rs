use shared::common::{AdapterName, AdapterNameList};

use shared::external_lint::IExternalLintSelectorProtocol;

pub struct CapabilitiesExternalLintSelector {
    rust_adapters: Vec<AdapterName>,
    python_adapters: Vec<AdapterName>,
    js_adapters: Vec<AdapterName>,
}

impl IExternalLintSelectorProtocol for CapabilitiesExternalLintSelector {
    fn select_adapters(&self, has_rs: bool, has_py: bool, has_js: bool) -> AdapterNameList {
        let mut adapter_names = Vec::new();
        if has_rs {
            for name in self.rust_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_py {
            for name in self.python_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_js {
            for name in self.js_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        AdapterNameList::new(adapter_names)
    }
}

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
