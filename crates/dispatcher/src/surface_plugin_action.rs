// PURPOSE: PluginCommandsSurface — adapter/plugin listing business logic, no formatting.
use shared_common::AdapterNameList;
use shared_common::taxonomy_adapter_name_vo::ToolName;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use std::sync::Arc;

pub fn collect_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> AdapterNameList {
    external_lint
        .execute(shared_external_lint::ExternalLintRequest::AdapterNames)
        .into_adapter_names()
}

/// Adapter with binary availability metadata — used by TUI for detailed display.
#[derive(Debug, Clone)]
pub struct AdapterDetail {
    pub name: String,
    pub label: String,
    pub installed: bool,
}

/// Discover all known adapters and check binary availability via filesystem aggregate.
pub fn collect_adapters_detailed(
    tool_resolution: &dyn IToolResolutionProtocol,
) -> Vec<AdapterDetail> {
    let mut list = vec![
        ("ast_rust_scanner", "Rust AST (built-in)", true),
        ("ast_py_scanner", "Python AST (built-in)", true),
        ("ast_js_scanner", "JS/TS AST (built-in)", true),
    ]
    .into_iter()
    .map(|(n, l, i)| AdapterDetail {
        name: n.into(),
        label: l.into(),
        installed: i,
    })
    .collect::<Vec<_>>();
    for (b, l) in [
        ("clippy", "Clippy (Rust)"),
        ("ruff", "Ruff (Python)"),
        ("mypy", "MyPy (Python)"),
        ("bandit", "Bandit (Python)"),
        ("radon", "Radon (Python metrics)"),
        ("eslint", "ESLint (JavaScript)"),
        ("prettier", "Prettier (JavaScript)"),
        ("tsc", "TypeScript Compiler"),
        ("markdownlint-cli", "markdownlint (Markdown)"),
    ] {
        list.push(AdapterDetail {
            name: b.into(),
            label: l.into(),
            installed: tool_resolution.is_binary_available(&ToolName::new(b)),
        });
    }
    list
}
