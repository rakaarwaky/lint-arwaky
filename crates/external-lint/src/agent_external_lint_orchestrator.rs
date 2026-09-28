// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator dynamically selects which adapters to run based on the
// languages detected in the project (Rust, Python, JavaScript/TypeScript,
// Markdown). It receives a pre-computed `ExternalLintContext` from the surface
// layer, eliminating all filesystem I/O from the agent layer (orchestration-only).
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
use shared::external_lint::contract_external_lint_protocol::IAdapterScanProtocol;
use shared::external_lint::contract_external_lint_protocol::ILanguageDetectProtocol;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared::external_lint::taxonomy_external_lint_request::ExternalLintRequest;
use shared::external_lint::taxonomy_external_lint_response::ExternalLintResponse;
use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintDeps {
    pub adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    pub selector: Arc<dyn IExternalLintSelectorProtocol>,
}

pub struct ExternalLintOrchestrator {
    deps: ExternalLintDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IExternalLintAggregate for ExternalLintOrchestrator {
    fn execute(&self, request: ExternalLintRequest) -> ExternalLintResponse {
        match request {
            ExternalLintRequest::ScanAll { path } => {
                let violations = self.scan_all(&path);
                ExternalLintResponse::Scan { violations }
            }
            ExternalLintRequest::ScanAllWithContext { path, context } => {
                let violations = self.scan_all_with_context(&path, &context);
                ExternalLintResponse::Scan { violations }
            }
            ExternalLintRequest::AdapterNames => ExternalLintResponse::AdapterNames {
                names: self.adapter_names(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ExternalLintOrchestrator {
    pub fn scan_all(&self, path: &FilePath) -> LintResultList {
        self.scan_all_with_context(path, &ExternalLintContext::default())
    }

    pub fn scan_all_with_context(
        &self,
        path: &FilePath,
        context: &ExternalLintContext,
    ) -> LintResultList {
        // Select adapters based on pre-computed language flags (no I/O).
        let selected: Vec<String> = self
            .deps
            .selector
            .select_adapters(
                context.has_rust,
                context.has_python,
                context.has_js,
                context.has_markdown,
            )
            .iter()
            .map(|a| a.value().to_string())
            .collect();

        // Filter by config entries if present (pre-computed by surface).
        let adapter_names: Vec<&str> = if context.config_entries.is_empty() {
            selected.iter().map(|s| s.as_str()).collect()
        } else {
            selected
                .iter()
                .filter(|name| {
                    context
                        .config_entries
                        .iter()
                        .any(|e| e.name.value() == **name)
                })
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
                    .filesystem_io
                    .should_ignore(&v.file, &context.ignored_paths)
            });
        }
        LintResultList::new(all)
    }
    pub fn adapter_names(&self) -> AdapterNameList {
        AdapterNameList::new(
            self.deps
                .adapters
                .keys()
                .map(|k| AdapterName::raw(k.clone()))
                .collect(),
        )
    }
    pub fn new(deps: ExternalLintDeps) -> Self {
        Self { deps }
    }
}

// ─── Block 4: FR-001 language detection ────────────────────

impl ILanguageDetectProtocol for ExternalLintOrchestrator {
    fn detect_languages(&self, path: &FilePath) -> (bool, bool, bool, bool) {
        let files = self
            .deps
            .filesystem
            .execute(FilesystemRequest::discover_files(std::path::Path::new(
                &path.value,
            )))
            .into_paths();
        let has_rust = files.iter().any(|f| f.ends_with(".rs"));
        let has_python = files.iter().any(|f| f.ends_with(".py"));
        let has_js = files.iter().any(|f| {
            f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx")
        });
        let has_markdown = files.iter().any(|f| f.ends_with(".md"));
        (has_rust, has_python, has_js, has_markdown)
    }
}

// ─── Block 5: FR-003 scan_all aggregation ─────────────────

impl IAdapterScanProtocol for ExternalLintOrchestrator {
    fn scan_all(&self, path: &FilePath, context: &ExternalLintContext) -> LintResultList {
        self.scan_all_with_context(path, context)
    }
}
