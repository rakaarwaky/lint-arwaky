// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use async_trait::async_trait;
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

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    dummy: Arc<dyn IDummyImportCheckerProtocol>,
    config: ArchitectureConfig,
    layer_map: LayerMapVO,
    ignored_paths: Vec<String>,
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

        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);

        let root_dir = shared::common::utility_file_handler::find_workspace_root(target.value())
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        let (mandatory_results, forbidden_results) = tokio::join!(
            async {
                let mut r = LintResultList::new(Vec::new());
                self.mandatory
                    .run_mandatory_imports(&self.config, &self.layer_map, &files, &root_dir, &mut r)
                    .await;
                r
            },
            async {
                let mut r = LintResultList::new(Vec::new());
                self.forbidden
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
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);

        for file in files.iter() {
            let file_path = file.value().to_string();
            let content =
                tokio::task::spawn_blocking(move || std::fs::read_to_string(&file_path).ok())
                    .await
                    .ok()
                    .flatten();
            if let Some(content) = content {
                self.unused
                    .check_unused_imports(file.value(), &content, &mut results.values);

                let content_str = ContentString::new(content);
                self.dummy.check_dummy_imports(
                    file,
                    &content_str,
                    &mut results.values,
                    &root_dir,
                    &self.layer_map,
                );
                self.dummy.check_dummy_functions(
                    file,
                    &content_str,
                    &mut results.values,
                    &root_dir,
                    &self.layer_map,
                );
                self.dummy.check_dummy_impls(
                    file,
                    &content_str,
                    &mut results.values,
                    &root_dir,
                    &self.layer_map,
                );
                self.dummy.check_taxonomy_intent(
                    file,
                    &content_str,
                    &mut results.values,
                    &root_dir,
                    &self.layer_map,
                );
                self.dummy.check_surface_logic(
                    file,
                    &content_str,
                    &mut results.values,
                    &root_dir,
                    &self.layer_map,
                );
            }
        }

        self.cycle
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
        mandatory: Arc<dyn IImportMandatoryProtocol>,
        forbidden: Arc<dyn IImportForbiddenProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleImportProtocol>,
        dummy: Arc<dyn IDummyImportCheckerProtocol>,
        config: ArchitectureConfig,
    ) -> Self {
        let layer_map = LayerMapVO::new(config.layers.clone());
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        Self {
            mandatory,
            forbidden,
            unused,
            cycle,
            dummy,
            config,
            layer_map,
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        if shared::common::utility_file_handler::is_path_ignored(&s, &self.ignored_paths) {
            return true;
        }
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if DEFAULT_SKIP_DIRS.contains(&dir_name.as_str()) {
            return true;
        }
        if let Some(stripped) = dir_name.strip_prefix('.') {
            return self.ignored_paths.iter().any(|i| i.contains(stripped));
        }
        false
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
