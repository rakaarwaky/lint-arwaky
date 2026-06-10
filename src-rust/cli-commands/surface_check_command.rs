// PURPOSE: Command: CLI surface for check — runs lint_path and resolves violations for a target path
use std::sync::Arc;

use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use std::process::ExitCode;

use crate::cli_commands::taxonomy_command_target_vo::{has_critical, lint_path, resolve_target};
use crate::output_report::taxonomy_result_vo::LintResultList;

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
        let path_obj = crate::source_parsing::taxonomy_path_vo::FilePath::new(path.to_string())
            .unwrap_or_else(|_| {
                crate::source_parsing::taxonomy_path_vo::FilePath::new(".".to_string())
                    .unwrap_or_default()
            });

        for name in &adapter_names {
            let adapter_name =
                crate::shared_common::taxonomy_adapter_name_vo::AdapterName::new(name.to_string())
                    .unwrap_or_default();
            if let Some(adapter) = container.linter_adapter(&adapter_name) {
                match rt.block_on(adapter.scan(&path_obj)) {
                    Ok(results) => {
                        all_results.extend(results.values);
                    }
                    Err(e) => {
                        eprintln!("[warn] {} adapter failed: {:?}", name, e);
                    }
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
            let results_list = LintResultList::new(all_results);
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

pub fn handle_check(path: Option<String>, _git_diff: bool) -> ExitCode {
    let root = resolve_target(path);
    run_lint_and_report(&root)
}

pub fn handle_scan(
    path: Option<String>,
    container: Arc<dyn ServiceContainerAggregate>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = register_check_commands(container);
    surface.scan(&root);
    ExitCode::SUCCESS
}

fn run_lint_and_report(root: &str) -> ExitCode {
    let results = lint_path(root);
    println!("=== AES Compliance Report for {} ===", root);
    for r in &results {
        println!(
            "[{}] {}:{}:{} {} - {}",
            r.severity, r.file, r.line, r.column, r.code, r.message
        );
    }
    println!("Total violations: {}", results.len());
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
