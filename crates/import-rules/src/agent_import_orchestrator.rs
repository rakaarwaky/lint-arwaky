// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
//
// Orchestrates 5 import-related AES rules by composing checker protocols.
// Each checker is injected via Arc<dyn Trait> — the orchestrator only
// knows about contract interfaces, never concrete implementations.
//
// Orchestration order:
//   1. AES202: mandatory imports check (files must import required symbols)
//   2. AES201: forbidden imports check (files must NOT import certain symbols)
//   3. AES204: dummy/intent import check (imports that exist only to satisfy
//      linters without being used)
//   4. AES203: unused import check (imports that are never referenced)
//   5. AES205: circular dependency detection
//
// Step 3 reuses the mandatory checker protocol (IArchImportProtocol) with a
// different configuration — the protocol is symmetric for both checks.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_constant::SOURCE_EXTENSIONS;
use shared::import_rules::taxonomy_import_utility::filepath_or_default;
use std::path::Path;
use std::sync::Arc;

/// Import orchestrator — the agent layer for import compliance.
///
/// Dependencies (all injected via `Arc<dyn Trait>`):
///   - `mandatory`: checks AES202 — required imports must be present
///   - `forbidden`: checks AES201 — prohibited imports must NOT be present
///   - `intent`: checks AES204 — imports that exist only to suppress linters
///   - `unused`: checks AES203 — imports that are never referenced
///   - `cycle`: checks AES205 — detects circular dependency chains
///   - `analyzer`: provides configuration (layer definitions, ignored paths, etc.)
///   - `helper`: provides filepath and string helper utilities

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    intent: Arc<dyn IDummyImportCheckerProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    analyzer: Arc<dyn ILayerDetectionProtocol>,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Public Contract ─────────────────────────────
#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = match p.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => String::new(),
        };
        FilePath::new(s.to_string())
            .unwrap_or_default()
            .is_ignored(&self.ignored_paths)
            || match dir_name.strip_prefix('.') {
                Some(n) => self.ignored_paths.iter().any(|i| i.contains(n)),
                None => false,
            }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.to_str().is_some_and(|e| SOURCE_EXTENSIONS.contains(&e)) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Run all 5 import-related AES checks on the target.
    ///
    /// Execution order matters:
    ///   1-3. Mandatory/forbidden/intent checks use the same protocol trait
    ///        (IArchImportProtocol) but with different rule configurations.
    ///   4. Unused import check reads each file individually (file I/O).
    ///   5. Cycle detection runs last — it requires the full import graph.
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        // Global gate: skip all import checks if architecture checker is disabled
        let config = self.analyzer.config();
        if !config.enabled.value {
            return Vec::new();
        }

        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let first_component = target.value().split('/').next().unwrap_or(".");
        let root_dir = filepath_or_default(
            FilePath::new(first_component.to_string()).map_err(|e| format!("{:?}", e)),
        );

        // Run mandatory/forbidden checks concurrently (no data sharing between them)
        let (mandatory_results, forbidden_results) = tokio::join!(
            async {
                let mut r = LintResultList::new(Vec::new());
                if config.is_rule_enabled("AES202") {
                    self.mandatory
                        .run_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut r)
                        .await;
                }
                r
            },
            async {
                let mut r = LintResultList::new(Vec::new());
                if config.is_rule_enabled("AES201") {
                    self.forbidden
                        .check_forbidden_imports(self.analyzer.as_ref(), &files, &root_dir, &mut r)
                        .await;
                }
                r
            }
        );
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);

        // AES204: dummy/intent import check — per-file check
        if config.is_rule_enabled("AES204") {
            for file in files.iter() {
                let file_path = file.value();
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    self.intent.check_dummy_imports(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                    self.intent.check_dummy_functions(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                    self.intent.check_dummy_impls(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                    self.intent.check_taxonomy_intent(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                    self.intent.check_layer_contract_intent(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                    self.intent.check_surface_logic(
                        file_path,
                        &content,
                        &mut results.values,
                        self.analyzer.as_ref(),
                        &root_dir,
                    );
                }
            }
        }

        // AES203: unused import check — read file content once and check all languages
        if config.is_rule_enabled("AES203") {
            for file in files.iter() {
                if let Ok(content) = std::fs::read_to_string(file.value()) {
                    self.unused
                        .check_unused_imports(file, &content, &mut results.values);
                }
            }
        }

        // AES205: circular dependency / cycle detection
        if config.is_rule_enabled("AES205") {
            self.cycle
                .check_cycles(self.analyzer.as_ref(), &files, &root_dir, &mut results)
                .await;
        }

        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ImportOrchestrator {
    /// Constructor: extracts ignored paths from config on initialization.
    /// This avoids repeated config lookups during file collection.
    pub fn new(
        mandatory: Arc<dyn IImportMandatoryProtocol>,
        forbidden: Arc<dyn IImportForbiddenProtocol>,
        intent: Arc<dyn IDummyImportCheckerProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleImportProtocol>,
        analyzer: Arc<dyn ILayerDetectionProtocol>,
    ) -> Self {
        let config = analyzer.config();
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            mandatory,
            forbidden,
            intent,
            unused,
            cycle,
            analyzer,
            ignored_paths,
        }
    }
}
