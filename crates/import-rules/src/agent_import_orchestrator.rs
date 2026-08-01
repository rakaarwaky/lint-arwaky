// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use async_trait::async_trait;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{ContentString, ErrorMessage, FilePath, FilePathList, ScanError};
use shared::filesystem::utility_filesystem_io::{path_exists, read_file, walk_source_files};

use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_constant::DEFAULT_SKIP_DIRS;

use shared::common::taxonomy_definition_vo::LayerMapVO;

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
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        if !self.config.enabled.value {
            return Ok(Vec::new());
        }
        if !path_exists(target.value()) {
            return Err(ScanError::new(
                FilePath::new(target.value().to_string()).unwrap_or_default(),
                ErrorMessage::new(format!("Target path does not exist: {}", target.value())),
            ));
        }

        let files = self.collect_files(target);

        let root_dir =
            shared::filesystem::utility_filesystem_io::find_workspace_root(target.value())
                .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
                .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        // Pre-read all file contents into a map so capabilities don't do I/O.
        let content_map: HashMap<String, String> = files
            .values
            .iter()
            .filter_map(|f| {
                read_file(f.value())
                    .ok()
                    .map(|c| (f.value().to_string(), c))
            })
            .collect();

        let (mandatory_result, forbidden_result) = tokio::join!(
            self.deps.mandatory.run_mandatory_imports(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &content_map,
            ),
            self.deps.forbidden.check_forbidden_imports(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &content_map,
            ),
        );
        let mandatory_results = mandatory_result?;
        let forbidden_results = forbidden_result?;

        let root_dir_clone = root_dir.clone();
        let deps = &self.deps;
        let layer_map = &self.layer_map;

        let file_violations: Vec<LintResult> =
            ParallelIterator::flat_map(IntoParallelRefIterator::par_iter(&files.values), |file| {
                let mut local_results = Vec::new();
                let content = match content_map.get(file.value()) {
                    Some(c) => c.clone(),
                    None => {
                        return local_results;
                    }
                };
                if let Ok(unused) = deps.unused.check_unused_imports(file.value(), &content) {
                    local_results.extend(unused);
                }

                let content_str = ContentString::new(content);
                if let Ok(dummy) =
                    deps.dummy
                        .check_all_dummy(file, &content_str, &root_dir_clone, layer_map)
                {
                    local_results.extend(dummy);
                }
                local_results
            })
            .collect();

        let mut results = LintResultList::new(Vec::new());
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);
        results.values.extend(file_violations);

        let cycle_violations = self
            .deps
            .cycle
            .check_cycles(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &content_map,
            )
            .await?;
        results.values.extend(cycle_violations);
        Ok(results.values)
    }

    fn name(&self) -> &str {
        "import-rules"
    }

    fn run_audit_with_entries(
        &self,
        files: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        // Build FilePathList and content_map from FileEntry
        let file_paths: Vec<FilePath> = files
            .iter()
            .filter(|f| f.parse_ok && !f.content.is_empty())
            .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
            .collect();
        let file_list = FilePathList::new(file_paths);

        let content_map: HashMap<String, String> = files
            .iter()
            .filter(|f| f.parse_ok)
            .map(|f| (f.path.to_string_lossy().to_string(), f.content.clone()))
            .collect();

        let root_dir = FilePath::new(".".to_string()).unwrap_or_default();
        let mut results = Vec::new();

        // Run async checks via tokio runtime
        let Ok(rt) = tokio::runtime::Runtime::new() else {
            return Vec::new();
        };
        rt.block_on(async {
            let mandatory_result = self
                .deps
                .mandatory
                .run_mandatory_imports(
                    &self.config,
                    &self.layer_map,
                    &file_list,
                    &root_dir,
                    &content_map,
                )
                .await;
            let forbidden_result = self
                .deps
                .forbidden
                .check_forbidden_imports(
                    &self.config,
                    &self.layer_map,
                    &file_list,
                    &root_dir,
                    &content_map,
                )
                .await;
            if let Ok(v) = mandatory_result {
                results.extend(v.values);
            }
            if let Ok(v) = forbidden_result {
                results.extend(v.values);
            }

            let cycle_result = self
                .deps
                .cycle
                .check_cycles(
                    &self.config,
                    &self.layer_map,
                    &file_list,
                    &root_dir,
                    &content_map,
                )
                .await;
            if let Ok(v) = cycle_result {
                results.extend(v);
            }
        });

        // Sync checks via rayon
        use rayon::prelude::*;
        let sync_violations: Vec<LintResult> = file_list
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local = Vec::new();
                let content_str = match content_map.get(file.value()) {
                    Some(c) => c.as_str(),
                    None => return local,
                };
                if let Ok(v) = self
                    .deps
                    .unused
                    .check_unused_imports(file.value(), content_str)
                {
                    local.extend(v);
                }
                let cs = ContentString::new(content_str.to_string());
                if let Ok(v) =
                    self.deps
                        .dummy
                        .check_all_dummy(file, &cs, &root_dir, &self.layer_map)
                {
                    local.extend(v);
                }
                local
            })
            .collect();
        results.extend(sync_violations);

        results
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

        Self {
            deps,
            config,
            layer_map,
            ignored_paths,
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            let mut ignored = self.ignored_paths.clone();
            for d in DEFAULT_SKIP_DIRS {
                let entry = format!("/{}", d);
                if !ignored.contains(&entry) {
                    ignored.push(entry);
                }
            }
            walk_source_files(path, &mut files, &ignored);
        } else if path.is_file() {
            match FilePath::new(path.to_string_lossy().to_string()) {
                Ok(fp) => files.push(fp),
                Err(e) => eprintln!(
                    "[warn] invalid file path '{}': {}",
                    path.to_string_lossy(),
                    e
                ),
            }
        }
        FilePathList::new(files)
    }
}
