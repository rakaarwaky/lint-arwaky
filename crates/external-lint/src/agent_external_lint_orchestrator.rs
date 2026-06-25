// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

pub struct ExternalLintOrchestrator {
    adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}

impl ExternalLintOrchestrator {
    pub fn new(adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>) -> Self {
        Self { adapters }
    }
}

#[async_trait]
impl IExternalLintAggregate for ExternalLintOrchestrator {
    async fn scan_all(&self, path: &FilePath) -> LintResultList {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        fn detect_languages(
            dir: &std::path::Path,
            has_rs: &mut bool,
            has_py: &mut bool,
            has_js: &mut bool,
        ) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = match path.file_name() {
                            Some(n) => n.to_string_lossy(),
                            None => continue,
                        };
                        if !matches!(
                            name.as_ref(),
                            "node_modules" | "target" | ".git" | ".jj" | "Graph-It-Live"
                        ) {
                            detect_languages(&path, has_rs, has_py, has_js);
                        }
                    } else if let Some(ext) = path.extension() {
                        match ext.to_str() {
                            Some("rs") => *has_rs = true,
                            Some("py") => *has_py = true,
                            Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                            _ => {}
                        }
                    }
                    if *has_rs && *has_py && *has_js {
                        break;
                    }
                }
            }
        }

        let root_path = std::path::Path::new(&path.value);
        if root_path.is_file() {
            if let Some(ext) = root_path.extension() {
                match ext.to_str() {
                    Some("rs") => has_rs = true,
                    Some("py") => has_py = true,
                    Some("js" | "ts" | "jsx" | "tsx") => has_js = true,
                    _ => {}
                }
            }
        } else {
            detect_languages(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        let mut adapter_names = Vec::new();
        if has_rs {
            adapter_names.push("clippy");
            adapter_names.push("rustfmt");
            adapter_names.push("cargo-audit");
        }
        if has_py {
            adapter_names.push("ruff");
            adapter_names.push("mypy");
            adapter_names.push("bandit");
        }
        if has_js {
            adapter_names.push("eslint");
            adapter_names.push("prettier");
            adapter_names.push("tsc");
        }

        let mut futures = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.adapters.get(*name) {
                let adapter: Arc<dyn ILinterAdapterPort> = adapter.clone();
                let path_clone = path.clone();
                let name_owned = name.to_string();
                futures.push(async move {
                    match adapter.scan(&path_clone).await {
                        Ok(results) => Ok::<Vec<_>, String>(results.values),
                        Err(e) => {
                            let err_msg = e.to_string();
                            if err_msg.contains("No such file or directory")
                                || err_msg.contains("os error 2")
                            {
                                eprintln!(
                                    "[warn] {} is not installed or not in system PATH. Skipping.",
                                    name_owned
                                );
                            } else {
                                eprintln!("[warn] {} adapter failed: {}", name_owned, err_msg);
                            }
                            Ok(Vec::new())
                        }
                    }
                });
            }
        }

        let results = future::join_all(futures).await;
        let mut all = Vec::new();
        for values in results.into_iter().flatten() {
            all.extend(values);
        }
        LintResultList::new(all)
    }

    fn adapter_names(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}
