// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator dynamically selects which adapters to run based on the
// languages detected in the project (Rust, Python, JavaScript/TypeScript).
// It performs a file-system scan to detect language usage before running
// any adapters — avoids running rustfmt on Python-only projects.
//
// Adapters are run concurrently via future::join_all. If an adapter's binary
// is not installed, a warning is printed (not an error) — the scan continues
// with the remaining adapters.
//
// Language detection uses async-aware directory scanning (tokio::fs) to avoid
// blocking the tokio runtime during recursive file-system traversal.
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::common::taxonomy_adapter_list_vo::AdapterNameList;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::utility_config_parser::parse_adapter_names_from_yaml;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintDeps {
    pub adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
}

pub struct ExternalLintOrchestrator {
    deps: ExternalLintDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

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
        ) -> std::io::Result<()> {
            let entries = match std::fs::read_dir(dir) {
                Ok(e) => e,
                Err(_) => return Ok(()),
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = match path.file_name().and_then(|n| n.to_str()) {
                        Some(n) => n,
                        None => continue,
                    };
                    if !matches!(
                        name,
                        "node_modules" | "target" | ".git" | ".jj" | "Graph-It-Live"
                    ) {
                        let _ = detect_languages(&path, has_rs, has_py, has_js);
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
            Ok(())
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
            let _ = detect_languages(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        let mut adapter_names = Vec::with_capacity(9);
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

        // Filter adapter_names by config's adapters section if a config YAML is found
        if let Some(configured_adapters) =
            load_configured_adapter_names(root_path, has_rs, has_py, has_js)
                .filter(|a| !a.is_empty())
        {
            adapter_names.retain(|name| configured_adapters.iter().any(|a| a == name));
        }

        let mut futures = Vec::with_capacity(9);
        for name in &adapter_names {
            if let Some(adapter) = self.deps.adapters.get(*name) {
                let adapter: Arc<dyn ILinterAdapterProtocol> = adapter.clone();
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
        let total_capacity: usize = results
            .iter()
            .filter_map(|r| r.as_ref().ok())
            .map(|v| v.len())
            .sum();
        let mut all = Vec::with_capacity(total_capacity);
        for values in results.into_iter().flatten() {
            all.extend(values);
        }
        LintResultList::new(all)
    }

    fn adapter_names(&self) -> AdapterNameList {
        AdapterNameList::new(
            self.deps
                .adapters
                .keys()
                .map(|k| AdapterName::raw(k.clone()))
                .collect(),
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ExternalLintOrchestrator {
    pub fn new(deps: ExternalLintDeps) -> Self {
        Self { deps }
    }
}

/// Walk up from `root_path` looking for lint_arwaky.config.*.yaml files.
/// Returns parsed adapter names if any config file is found, else None.
fn load_configured_adapter_names(
    root_path: &Path,
    has_rs: bool,
    has_py: bool,
    has_js: bool,
) -> Option<Vec<String>> {
    let config_names: Vec<String> = {
        let mut names = Vec::new();
        names.push("lint_arwaky.config.yaml".to_string());
        if has_js {
            names.push("lint_arwaky.config.javascript.yaml".to_string());
        }
        if has_py {
            names.push("lint_arwaky.config.python.yaml".to_string());
        }
        if has_rs {
            names.push("lint_arwaky.config.rust.yaml".to_string());
        }
        names
    };

    let start = if root_path.is_file() {
        root_path.parent().unwrap_or(root_path)
    } else {
        root_path
    };

    let mut current: Option<&Path> = Some(start);
    while let Some(dir) = current {
        for cfg_name in &config_names {
            let cfg_path = dir.join(cfg_name);
            if cfg_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&cfg_path) {
                    let adapters = parse_adapter_names_from_yaml(&content);
                    if !adapters.is_empty() {
                        return Some(adapters);
                    }
                }
            }
        }
        // Do not walk up past the root
        current = dir.parent().filter(|&p| p != dir);
    }

    None
}
