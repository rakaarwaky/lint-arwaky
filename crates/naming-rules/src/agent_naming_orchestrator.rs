// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_definition_vo::LayerMapVO;
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
    ignored_patterns: PatternList,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        let target_path = Path::new(&target.value);

        if !target_path.exists() {
            return Err(ScanError::new(
                target.clone(),
                ErrorMessage::new("target path does not exist"),
            ));
        }

        let mut results = LintResultList::new(Vec::new());
        let all_files = shared::naming_rules::utility_naming_filesystem::walk_recursive(
            target,
            Some(&self.ignored_patterns),
        );
        let files = shared::naming_rules::utility_file_filter::filter_source_files(&all_files);

        self.deps
            .naming_convention_checker
            .check_file_naming(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                &files,
                target,
                &mut results,
            )
            .await;
        self.deps
            .suffix_prefix_checker
            .check_domain_suffixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                &files,
                target,
                &mut results,
            )
            .await;

        Ok(results.values)
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl NamingOrchestrator {
    pub fn new(deps: NamingOrchestratorDeps) -> Self {
        let ignored_patterns = PatternList {
            values: deps
                .config
                .ignored_paths
                .values
                .iter()
                .map(|fp| {
                    fp.value
                        .trim_start_matches("./")
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .to_string()
                })
                .collect(),
        };
        Self {
            deps,
            ignored_patterns,
        }
    }
}
