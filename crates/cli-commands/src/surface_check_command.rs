// PURPOSE: Command: CLI surface for check — runs lint_path and resolves violations for a target path
use std::sync::Arc;

use futures::future;
use shared::common::contract_service_aggregate::ServiceContainerAggregate;
use std::process::ExitCode;

use code_analysis::{has_critical, lint_path, resolve_target};
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;

pub struct CheckCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl Default for CheckCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    /// scan: Full multi-language lint on target project path.
    /// Uses all available linter adapters via tokio runtime.
    pub fn scan(&self, path: &str) {
        self.scan_filtered(path, None);
    }

    /// scan_filtered: scan with optional AES rule code filter.
    pub fn scan_filtered(&self, path: &str, filter: Option<&str>) {
        let container = match self.container.as_ref() {
            Some(c) => c.clone(),
            None => {
                eprintln!("[error] container not registered");
                return;
            }
        };

        let mut all_results = Vec::new();
        let adapter_names = [
            "clippy",
            "rustfmt",
            "cargo-audit",
            "ruff",
            "mypy",
            "bandit",
            "eslint",
            "prettier",
            "tsc",
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
            if let Some(adapter) = container.linter_adapter(&adapter_name) {
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
                Ok((_name, values)) => {
                    all_results.extend(values);
                }
                Err((name, e)) => {
                    eprintln!("[warn] {} adapter failed: {}", name, e);
                }
            }
        }

        // AES architecture lint via ServiceContainerAggregate contract (no direct agent import)
        let has_src = ["src-rust", "src-python", "src-javascript", "src"]
            .iter()
            .any(|d| std::path::Path::new(path).join(d).is_dir());
        if let Some(linter) = container.get_architecture_linter() {
            if has_src {
                let aes_results = linter.run_self_lint(path);
                all_results.extend(aes_results.values);
            }
            let filtered_results: Vec<_> = if let Some(code) = filter {
                all_results
                    .into_iter()
                    .filter(|r| r.code.to_string().contains(code))
                    .collect()
            } else {
                all_results
            };
            let results_list = LintResultList::new(filtered_results);
            println!("{}", linter.format_report(&results_list, path));
        }
    }
}

pub fn register_check_commands(
    container: Arc<dyn ServiceContainerAggregate>,
) -> CheckCommandsSurface {
    let mut surface = CheckCommandsSurface::new();
    surface.register_all(container);
    surface
}

pub fn handle_check(path: Option<String>, _git_diff: bool, filter: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    run_lint_and_report(&root, filter.as_deref())
}

pub fn handle_scan(
    path: Option<String>,
    container: Arc<dyn ServiceContainerAggregate>,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = register_check_commands(container);
    surface.scan_filtered(&root, filter.as_deref());
    ExitCode::SUCCESS
}

fn run_lint_and_report(root: &str, filter: Option<&str>) -> ExitCode {
    let results = lint_path(root);
    println!("=== AES Compliance Report for {} ===", root);
    let filtered: Vec<_> = if let Some(code) = filter {
        results
            .iter()
            .filter(|r| r.code.to_string().contains(code))
            .collect()
    } else {
        results.iter().collect()
    };
    for r in &filtered {
        println!(
            "[{}] {}:{}:{} {} - {}",
            r.severity, r.file, r.line, r.column, r.code, r.message
        );
    }
    println!("Total violations: {}", filtered.len());
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
