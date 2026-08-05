// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
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
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        // Naming checks are path-only — do NOT skip parse failures.
        // Per FRD glossary, skip only UNREADABLE files (empty content).
        let file_paths: Vec<FilePath> = files
            .iter()
            .filter(|f| !f.content.is_empty())
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

    /// Check if a specific AES rule is enabled in the configuration.
    /// Returns true if the rule is found and enabled, or if not found (default enabled).
    fn is_rule_enabled(config: &ArchitectureConfig, rule_code: &str) -> bool {
        config
            .rules
            .iter()
            .find(|r| r.rule_type.code() == rule_code)
            .is_none_or(|r| r.enabled.value)
    }

    fn run_checks(&self, files: &FilePathList, root_dir: &FilePath) -> Vec<LintResult> {
        let mut results: Vec<LintResult> = Vec::new();

        if Self::is_rule_enabled(&self.deps.config, "AES101") {
            let mut naming_results = LintResultList::new(Vec::new());
            self.deps.naming_convention_checker.check_file_naming(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut naming_results,
            );
            results.extend(naming_results.values);
        }

        if Self::is_rule_enabled(&self.deps.config, "AES102") {
            let mut suffix_results = LintResultList::new(Vec::new());
            self.deps.suffix_prefix_checker.check_domain_suffixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut suffix_results,
            );
            results.extend(suffix_results.values);
        }

        results
    }
}
