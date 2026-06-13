// PURPOSE: Command: CLI surface for check/scan — runs AES analysis on target path
use std::sync::Arc;

use futures::future;
use std::process::ExitCode;

use code_analysis::resolve_target;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;

pub struct CheckCommandsSurface {
    pub linter_adapters: std::collections::HashMap<String, Arc<dyn ILinterAdapterPort>>,
    pub arch_linter: Arc<dyn IArchLintProtocol>,
}

impl CheckCommandsSurface {
    pub fn new(
        linter_adapters: std::collections::HashMap<String, Arc<dyn ILinterAdapterPort>>,
        arch_linter: Arc<dyn IArchLintProtocol>,
    ) -> Self {
        Self { linter_adapters, arch_linter }
    }

    /// Run AES analysis + external adapters on a target path.
    pub fn scan(&self, path: &str, filter: Option<&str>) {
        let mut all_results = Vec::new();

        // 1. Run AES analysis (same algorithm for check and scan)
        let aes_results = self.arch_linter.run_self_lint(path);
        all_results.extend(aes_results.values);

        // 2. Run external linter adapters
        let adapter_names = [
            "clippy", "rustfmt", "cargo-audit", "ruff", "mypy", "bandit", "eslint", "prettier", "tsc",
        ];
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };
        let path_obj = FilePath::new(path.to_string())
            .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default());

        let mut adapter_futures = Vec::new();
        for name in &adapter_names {
            let adapter_name = match AdapterName::new(name.to_string()) {
                Ok(a) => a,
                Err(_) => continue,
            };
            if let Some(adapter) = self.linter_adapters.get(adapter_name.value()).cloned() {
                let path_clone = path_obj.clone();
                let name_owned = name.to_string();
                adapter_futures.push(async move {
                    match adapter.scan(&path_clone).await {
                        Ok(results) => Ok((name_owned, results.values)),
                        Err(e) => Err((name_owned, format!("{:?}", e))),
                    }
                });
            }
        }

        let adapter_results = rt.block_on(future::join_all(adapter_futures));
        for result in adapter_results {
            match result {
                Ok((_name, values)) => all_results.extend(values),
                Err((name, e)) => eprintln!("[warn] {} adapter failed: {}", name, e),
            }
        }

        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results.into_iter().filter(|r| r.code.to_string().contains(code)).collect()
        } else {
            all_results
        };
        let results_list = LintResultList::new(filtered_results);
        println!("{}", self.arch_linter.format_report(&results_list, path));
    }
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<String>,
    _git_diff: bool,
    arch_linter: Arc<dyn IArchLintProtocol>,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = CheckCommandsSurface::new(std::collections::HashMap::new(), arch_linter);
    surface.scan(&root, filter.as_deref());
    ExitCode::SUCCESS
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<String>,
    linter_adapters: std::collections::HashMap<String, Arc<dyn ILinterAdapterPort>>,
    arch_linter: Arc<dyn IArchLintProtocol>,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = CheckCommandsSurface::new(linter_adapters, arch_linter);
    surface.scan(&root, filter.as_deref());
    ExitCode::SUCCESS
}

