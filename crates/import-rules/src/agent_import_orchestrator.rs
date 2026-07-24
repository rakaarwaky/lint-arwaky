// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use async_trait::async_trait;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_source_vo::ContentString;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_constant::DEFAULT_SKIP_DIRS;
use shared::taxonomy_definition_vo::LayerMapVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ImportOrchestratorDeps {
    pub mandatory: Arc<dyn IImportMandatoryProtocol>,
    pub forbidden: Arc<dyn IImportForbiddenProtocol>,
    pub unused: Arc<dyn IUnusedImportProtocol>,
    pub cycle: Arc<dyn ICycleImportProtocol>,
    pub dummy: Arc<dyn IDummyImportCheckerProtocol>,
}

pub struct ImportOrchestrator {
    deps: ImportOrchestratorDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
    aes201_exceptions: HashSet<String>,
    aes202_exceptions: HashSet<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        if !self.config.enabled.value {
            return Ok(Vec::new());
        }
        let path = Path::new(target.value());
        if !path.exists() {
            return Err(ScanError::new(
                FilePath::new(target.value().to_string()).unwrap_or_default(),
                ErrorMessage::new(format!("Target path does not exist: {}", target.value())),
            ));
        }

        let files = self.collect_files(target);

        let root_dir = shared::common::utility_file_handler::find_workspace_root(target.value())
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        let (mandatory_results, forbidden_results) = tokio::join!(
            async {
                let mut r = LintResultList::new(Vec::new());
                self.deps
                    .mandatory
                    .run_mandatory_imports(&self.config, &self.layer_map, &files, &root_dir, &mut r)
                    .await;
                r
            },
            async {
                let mut r = LintResultList::new(Vec::new());
                self.deps
                    .forbidden
                    .check_forbidden_imports(
                        &self.config,
                        &self.layer_map,
                        &files,
                        &root_dir,
                        &mut r,
                    )
                    .await;
                r
            }
        );

        let root_dir_clone = root_dir.clone();
        let deps = &self.deps;
        let layer_map = &self.layer_map;

        let file_violations: Vec<LintResult> = files
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local_results = Vec::new();
                if let Ok(content) = std::fs::read_to_string(file.value()) {
                    deps.unused.check_unused_imports(
                        file.value(),
                        &content,
                        &mut local_results,
                    );

                    let content_str = ContentString::new(content);
                    deps.dummy.check_all_dummy(
                        file,
                        &content_str,
                        &mut local_results,
                        &root_dir_clone,
                        layer_map,
                    );
                }
                local_results
            })
            .collect();

        let mut results = LintResultList::new(Vec::new());
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);
        results.values.extend(file_violations);

        self.deps
            .cycle
            .check_cycles(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &mut results,
            )
            .await;
        Ok(results.values)
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ImportOrchestrator {
    pub fn new(
        deps: ImportOrchestratorDeps,
        config: ArchitectureConfig,
        ignored_paths: Vec<String>,
    ) -> Self {
        let layer_map = LayerMapVO::new(config.layers.clone());

        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        Self {
            deps,
            config,
            layer_map,
            ignored_paths,
            aes201_exceptions,
            aes202_exceptions,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if DEFAULT_SKIP_DIRS.contains(&dir_name.as_str()) || dir_name.starts_with('.') {
            return true;
        }
        let path_str = p.to_string_lossy();
        self.ignored_paths.iter().any(|ignored| {
            path_str.contains(ignored.as_str())
                || dir_name.contains(ignored.trim_start_matches('/'))
        })
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, false);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if is_subdir && self.is_ignored(dir) {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if self.is_ignored(&path) {
                        continue;
                    }
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
                        }
                    }
                }
            }
        }
    }
}
