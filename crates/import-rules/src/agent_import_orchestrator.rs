// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use std::sync::Arc;

use shared::common::utility_file_handler::{path_exists, read_file_sync, walk_source_files};
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{ContentString, ErrorMessage, FilePath, FilePathList, ScanError};

use shared::config_system::ArchitectureConfig;
use shared::import_rules::{
    ICycleImportProtocol, IDummyImportCheckerProtocol, IImportForbiddenProtocol,
    IImportMandatoryProtocol, IImportRunnerAggregate, IUnusedImportProtocol, DEFAULT_SKIP_DIRS,
};

use shared::common::LayerMapVO;

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

        let root_dir = shared::common::utility_file_handler::find_workspace_root(target.value())
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        let (mandatory_result, forbidden_result) = tokio::join!(
            self.deps.mandatory.run_mandatory_imports(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir
            ),
            self.deps.forbidden.check_forbidden_imports(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir
            ),
        );
        let mandatory_results = mandatory_result?;
        let forbidden_results = forbidden_result?;

        let root_dir_clone = root_dir.clone();
        let deps = &self.deps;
        let layer_map = &self.layer_map;

        let file_violations: Vec<LintResult> = files
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local_results = Vec::new();
                let content = match read_file_sync(file.value()) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("[warn] skipping unreadable file '{}': {}", file.value(), e);
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
            .check_cycles(&self.config, &self.layer_map, &files, &root_dir)
            .await?;
        results.values.extend(cycle_violations);
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
