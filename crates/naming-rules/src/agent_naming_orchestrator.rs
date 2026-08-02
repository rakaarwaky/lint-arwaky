// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::{INamingConventionChecker, ISuffixPrefixChecker};
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct NamingOrchestratorDeps {
    pub naming_convention_checker: Arc<dyn INamingConventionChecker>,
    pub suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    pub config: Arc<ArchitectureConfig>,
    pub layer_map: Arc<LayerMapVO>,
}

pub struct NamingOrchestrator {
    deps: NamingOrchestratorDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl INamingRunnerAggregate for NamingOrchestrator {
    fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        let target_path = Path::new(&target.value);

        if !target_path.exists() {
            return Err(ScanError::new(
                target.clone(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "target path does not exist",
                ),
            ));
        }

        // NOTE: run_audit is the simplified entry point. The caller should
        // use run_audit_with_entries with pre-discovered file entries for
        // full control over file discovery.
        Err(ScanError::new(
            target.clone(),
            shared::common::taxonomy_common_error::ErrorMessage::new(
                "use run_audit_with_entries with pre-discovered file entries",
            ),
        ))
    }

    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        // Convert FileEntry paths to FilePathList for the checkers
        let file_paths: Vec<FilePath> = files
            .iter()
            .filter(|f| f.parse_ok && !f.content.is_empty())
            .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
            .collect();
        let file_list = FilePathList::new(file_paths);
        let root = FilePath::new(".".to_string()).unwrap_or_default();

        self.run_checks(&file_list, &root)
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl NamingOrchestrator {
    pub fn new(deps: NamingOrchestratorDeps) -> Self {
        Self { deps }
    }

    fn run_checks(&self, files: &FilePathList, root_dir: &FilePath) -> Vec<LintResult> {
        let mut naming_results = LintResultList::new(Vec::new());
        let mut suffix_results = LintResultList::new(Vec::new());

        self.deps.naming_convention_checker.check_file_naming(
            self.deps.config.as_ref(),
            self.deps.layer_map.as_ref(),
            files,
            root_dir,
            &mut naming_results,
        );

        self.deps.suffix_prefix_checker.check_domain_suffixes(
            self.deps.config.as_ref(),
            self.deps.layer_map.as_ref(),
            files,
            root_dir,
            &mut suffix_results,
        );

        naming_results.values.extend(suffix_results.values);
        naming_results.values
    }
}
