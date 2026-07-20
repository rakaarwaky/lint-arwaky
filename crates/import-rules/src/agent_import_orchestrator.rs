// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::path::Path;
use std::sync::Arc;

pub fn str_or<'a>(opt: Option<&'a str>, fallback: &'a str) -> &'a str {
    opt.map_or(fallback, |s| s)
}

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    config: ArchitectureConfig,
    layer_map: LayerMapVO,
    ignored_paths: Vec<String>,
}

impl ImportOrchestrator {
    pub fn new(
        mandatory: Arc<dyn IImportMandatoryProtocol>,
        forbidden: Arc<dyn IImportForbiddenProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleImportProtocol>,
    ) -> Self {
        let (merged_layers, _) = shared::config_system::utility_config_merger::merge_config(
            &ArchitectureConfig::default(),
        );
        let config = ArchitectureConfig::default();
        let layer_map = LayerMapVO::new(merged_layers.clone());
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            mandatory,
            forbidden,
            unused,
            cycle,
            config,
            layer_map,
            ignored_paths,
        }
    }

    pub fn with_config(config: ArchitectureConfig) -> Self {
        let (merged_layers, _) =
            shared::config_system::utility_config_merger::merge_config(&config);
        let mut config = config;
        config.layers = merged_layers.clone();
        let layer_map = LayerMapVO::new(merged_layers);
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let empty_mandatory: Arc<dyn IImportMandatoryProtocol> = Arc::new(
            crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
        );
        let empty_forbidden: Arc<dyn IImportForbiddenProtocol> = Arc::new(
            crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(),
        );
        let empty_unused: Arc<dyn IUnusedImportProtocol> =
            Arc::new(crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new());
        let empty_cycle: Arc<dyn ICycleImportProtocol> =
            Arc::new(crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new());
        Self {
            mandatory: empty_mandatory,
            forbidden: empty_forbidden,
            unused: empty_unused,
            cycle: empty_cycle,
            config,
            layer_map,
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        shared::common::utility_file::is_path_ignored(&s, &self.ignored_paths)
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

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let first_component = str_or(target.value().split('/').next(), ".");
        let root_dir = match FilePath::new(first_component.to_string()) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

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
            let file_path = file.value();
            if let Ok(content) = std::fs::read_to_string(file_path) {
                self.unused
                    .check_unused_imports(file_path, &content, &mut results.values);
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
        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}
