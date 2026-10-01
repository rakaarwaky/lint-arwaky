// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator selects which adapters to run from language flags the caller
// supplies, then runs them sequentially. It receives a pre-computed
// `ExternalLintContext` from the surface layer, eliminating all filesystem I/O
// from the agent layer (orchestration-only). When no context is supplied the
// agent asks the `filesystem` aggregate for the flags — it never walks the tree
// itself (FR-Filesystem-005).
//
// Adapters are run sequentially. If an adapter's binary
// is not installed, a warning is printed (not an error) — the scan continues
// with the remaining adapters.
use std::collections::HashMap;
use std::sync::Arc;

use shared_cli_commands::taxonomy_result_vo::LintResultList;
use shared_common::AdapterNameList;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_path_vo::FilePath;
use shared_external_lint::IExternalLintAggregate;
use shared_external_lint::IExternalLintSelectorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_external_lint_request::ExternalLintRequest;
use shared_external_lint::taxonomy_external_lint_response::ExternalLintResponse;
use shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO;
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
    pub fn new(deps: ExternalLintDeps) -> Self {
        Self { deps }
    }

    /// Scan `path` with no caller-supplied context.
    ///
    /// The language flags come from the `filesystem` aggregate, which owns
    /// project-level language detection (FR-Filesystem-005). external-lint
    /// borrows the capability rather than re-implementing the extension walk.
    pub fn scan_all(&self, path: &FilePath) -> LintResultList {
        let languages = self
            .deps
            .filesystem
            .execute(FilesystemRequest::detect_project_languages(
                std::path::Path::new(&path.value),
            ))
            .into_project_languages();
        // Nothing recognisable under the root means no adapter applies, so stop
        // before building a context that would select nothing anyway.
        if languages.is_empty() {
            return LintResultList::new(Vec::new());
        }
        let context = self.context_from_languages(&languages);
        self.scan_all_with_context(path, &context)
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

    /// Build a scan context from the language flags the `filesystem` aggregate
    /// reported for a project root.
    ///
    /// The extension walk is the filesystem feature's capability, so the agent
    /// only reads the returned flags and shapes them into a context.
    fn context_from_languages(&self, languages: &ProjectLanguagesVO) -> ExternalLintContext {
        ExternalLintContext {
            has_rust: languages.has_rust,
            has_python: languages.has_python,
            has_js: languages.has_js,
            has_markdown: languages.has_markdown,
            // Built-in ignore list, so a caller that delegates the whole context
            // to this agent still gets findings filtered out of `target`,
            // `node_modules`, `tests` and the rest. An empty list would skip
            // the post-scan filter entirely, since the filter only runs when
            // there is something to filter against.
            ignored_paths: shared_common::DEFAULT_IGNORED_PATHS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            config_entries: Vec::new(),
        }
    }
}
