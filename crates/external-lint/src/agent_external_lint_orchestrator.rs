// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator dynamically selects which adapters to run based on the
// languages detected in the project (Rust, Python, JavaScript/TypeScript).
// It receives a pre-computed `ExternalLintContext` from the surface layer,
// eliminating all filesystem I/O from the agent layer (orchestration-only).
//
// Adapters are run sequentially. If an adapter's binary
// is not installed, a warning is printed (not an error) — the scan continues
// with the remaining adapters.
use std::collections::HashMap;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::AdapterNameList;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::IExternalLintAggregate;
use shared::external_lint::IExternalLintSelectorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintDeps {
    pub adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub selector: Arc<dyn IExternalLintSelectorProtocol>,
}

pub struct ExternalLintOrchestrator {
    deps: ExternalLintDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IExternalLintAggregate for ExternalLintOrchestrator {
    fn scan_all(&self, path: &FilePath) -> LintResultList {
        self.scan_all_with_context(path, &ExternalLintContext::default())
    }

    fn scan_all_with_context(
        &self,
        path: &FilePath,
        context: &ExternalLintContext,
    ) -> LintResultList {
        // Select adapters based on pre-computed language flags (no I/O).
        let selected: Vec<String> = self
            .deps
            .selector
            .select_adapters(context.has_rust, context.has_python, context.has_js)
            .iter()
            .map(|a| a.value().to_string())
            .collect();

        // Filter by config entries if present (pre-computed by surface).
        let adapter_names: Vec<&str> = if context.config_entries.is_empty() {
            selected.iter().map(|s| s.as_str()).collect()
        } else {
            selected
                .iter()
                .filter(|name| context.config_entries.iter().any(|e| e.name.value() == **name))
                .map(|s| s.as_str())
                .collect()
        };

        // Run adapters sequentially (this is the actual orchestration work).
        let mut all = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.deps.adapters.get(*name) {
                match adapter.scan(path) {
                    Ok(results) => {
                        all.extend(results.values);
                    }
                    Err(e) => {
                        let err_msg = e.to_string();
                        if err_msg.contains("No such file or directory")
                            || err_msg.contains("os error 2")
                        {
                            warn!(
                                adapter = name,
                                "is not installed or not in system PATH. Skipping."
                            );
                        } else {
                            warn!(
                                adapter = name,
                                error = %err_msg,
                                "adapter failed"
                            );
                        }
                    }
                }
            }
        }

        // Post-processing: filter violations by pre-computed ignored paths.
        // should_ignore() is a read-only check on already-computed data,
        // not filesystem I/O, so it remains in the orchestrator.
        if !context.ignored_paths.is_empty() {
            all.retain(|v| {
                !self
                    .deps
                    .filesystem
                    .should_ignore(&v.file, &context.ignored_paths)
            });
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
