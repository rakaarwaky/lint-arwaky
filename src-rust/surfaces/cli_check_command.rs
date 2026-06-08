use std::sync::Arc;

use crate::contract::service_container_aggregate::ServiceContainerAggregate;
use crate::taxonomy::LintResultList;

pub struct CheckCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
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
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        let path_obj = crate::taxonomy::FilePath::new(path.to_string()).unwrap_or_else(|_| {
            crate::taxonomy::FilePath::new(".".to_string()).unwrap_or_default()
        });

        for name in &adapter_names {
            let adapter_name =
                crate::taxonomy::AdapterName::new(name.to_string()).unwrap_or_default();
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
