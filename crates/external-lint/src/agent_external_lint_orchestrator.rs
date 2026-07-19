// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator uses DI to:
//   1. Detect languages via IExternalLintLanguageDetectorPort (infrastructure)
//   2. Select adapters via IExternalLintSelectorProtocol (capabilities)
//   3. Run adapters concurrently via future::join_all (agent flow control)
//
// If an adapter's binary is not installed, a warning is printed (not an error) — the scan continues.
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_language_detector_port::IExternalLintLanguageDetectorPort;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ExternalLintOrchestrator {
    adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
    language_detector: Arc<dyn IExternalLintLanguageDetectorPort>,
    selector: Arc<dyn IExternalLintSelectorProtocol>,
}

#[async_trait]
// ─── Block 2: Public Contract ─────────────────────────────
impl IExternalLintAggregate for ExternalLintOrchestrator {
    async fn scan_all(&self, path: &FilePath) -> LintResultList {
        let detected = self.language_detector.detect_languages(path).await;
        let adapter_names =
            self.selector
                .select_adapters(detected.has_rs, detected.has_py, detected.has_js);

        let mut futures = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.adapters.get(name.as_str()) {
                let adapter: Arc<dyn ILinterAdapterPort> = adapter.clone();
                let path_clone = path.clone();
                let name_owned = name.clone();
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
// ─── Block 3: Constructors & Helpers ──────────────────────
impl ExternalLintOrchestrator {
    pub fn new(
        adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
        language_detector: Arc<dyn IExternalLintLanguageDetectorPort>,
        selector: Arc<dyn IExternalLintSelectorProtocol>,
    ) -> Self {
        Self {
            adapters,
            language_detector,
            selector,
        }
    }
}
