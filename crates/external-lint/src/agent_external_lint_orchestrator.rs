// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator runs adapter scans sequentially and post-filters results by
// ignored paths. Language detection and adapter selection are private mechanics
// (not exposed as protocols); the surface layer may supply a pre-computed
// context via `ScanAllWithContext`, or the orchestrator will detect languages
// itself when called via `ScanAll`.
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
use shared::external_lint::contract_external_lint_protocol::IAdapterScanProtocol;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared::external_lint::taxonomy_external_lint_request::ExternalLintRequest;
use shared::external_lint::taxonomy_external_lint_response::ExternalLintResponse;
use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────

/// Default adapter groups keyed by language.
#[derive(Clone, Debug)]
pub struct AdapterGroups {
    pub rust: Vec<AdapterName>,
    pub python: Vec<AdapterName>,
    pub js: Vec<AdapterName>,
    pub markdown: Vec<AdapterName>,
}

pub struct ExternalLintDeps {
    pub adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    /// Optional language-specific adapter overrides from config.
    /// When present, these replace the default groups.
    pub adapter_groups: Option<AdapterGroups>,
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
        let context = ExternalLintContext {
            has_rust: self.detect_languages(path).0,
            has_python: self.detect_languages(path).1,
            has_js: self.detect_languages(path).2,
            has_markdown: self.detect_languages(path).3,
            ..Default::default()
        };
        self.scan_all_with_context(path, &context)
    }

    pub fn scan_all_with_context(
        &self,
        path: &FilePath,
        context: &ExternalLintContext,
    ) -> LintResultList {
        // Select adapters from config entries or pre-computed language flags.
        let selected: Vec<String> = if context.config_entries.is_empty() {
            let groups = self
                .deps
                .adapter_groups
                .clone()
                .unwrap_or_else(ExternalLintOrchestrator::new_default_groups);
            let rust_on = context.has_rust || self.detect_languages(path).0;
            let python_on = context.has_python || self.detect_languages(path).1;
            let js_on = context.has_js || self.detect_languages(path).2;
            let md_on = context.has_markdown || self.detect_languages(path).3;
            let mut names = Vec::new();
            if rust_on {
                names.extend(groups.rust.iter().map(|a| a.value().to_string()));
            }
            if python_on {
                names.extend(groups.python.iter().map(|a| a.value().to_string()));
            }
            if js_on {
                names.extend(groups.js.iter().map(|a| a.value().to_string()));
            }
            if md_on {
                names.extend(groups.markdown.iter().map(|a| a.value().to_string()));
            }
            names
        } else {
            // Config-driven: select from full adapter map filtered by config entries.
            let all_names: Vec<String> = self.deps.adapters.keys().cloned().collect();
            all_names
                .into_iter()
                .filter(|name| {
                    context
                        .config_entries
                        .iter()
                        .any(|e| e.name.value() == name.as_str())
                })
                .collect()
        };

        // Run adapters sequentially (this is the actual orchestration work).
        let mut all = Vec::new();
        for name in &selected {
            if let Some(adapter) = self.deps.adapters.get(name.as_str()) {
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

    /// Detect languages via extension walk over `path`. Private — not a protocol.
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

    /// Return the default adapter groups keyed by language.
    pub fn new_default_groups() -> AdapterGroups {
        AdapterGroups {
            rust: vec![
                AdapterName::raw("clippy"),
                AdapterName::raw("rustfmt"),
                AdapterName::raw("cargo-audit"),
            ],
            python: vec![
                AdapterName::raw("ruff"),
                AdapterName::raw("mypy"),
                AdapterName::raw("bandit"),
            ],
            js: vec![
                AdapterName::raw("eslint"),
                AdapterName::raw("prettier"),
                AdapterName::raw("tsc"),
            ],
            markdown: vec![AdapterName::raw("markdownlint")],
        }
    }
}

// ─── Block 4: scan_all aggregation (FR-001) ───────────────

impl IAdapterScanProtocol for ExternalLintOrchestrator {
    fn scan_all(&self, path: &FilePath, context: &ExternalLintContext) -> LintResultList {
        self.scan_all_with_context(path, context)
    }
}
