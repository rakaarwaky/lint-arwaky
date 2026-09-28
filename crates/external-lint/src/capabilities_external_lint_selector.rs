use shared::common::AdapterNameList;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::external_lint::contract_external_lint_protocol::IExternalLintSelectorProtocol;

pub struct CapabilitiesExternalLintSelector {
    rust_adapters: Vec<AdapterName>,
    python_adapters: Vec<AdapterName>,
    js_adapters: Vec<AdapterName>,
    markdown_adapters: Vec<AdapterName>,
}

impl IExternalLintSelectorProtocol for CapabilitiesExternalLintSelector {
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
        has_md: bool,
    ) -> AdapterNameList {
        let mut adapter_names = Vec::new();
        for (present, group) in [
            (has_rs, &self.rust_adapters),
            (has_py, &self.python_adapters),
            (has_js, &self.js_adapters),
            (has_md, &self.markdown_adapters),
        ] {
            if present {
                adapter_names.extend(group.iter().cloned());
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
        markdown_adapters: Vec<AdapterName>,
    ) -> Self {
        Self {
            rust_adapters,
            python_adapters,
            js_adapters,
            markdown_adapters,
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
            vec![AdapterName::raw("markdownlint")],
        )
    }
}
