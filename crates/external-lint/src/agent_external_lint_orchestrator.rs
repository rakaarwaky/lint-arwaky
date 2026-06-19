// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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
                            eprintln!("[warn] {} adapter failed: {:?}", name_owned, e);
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
