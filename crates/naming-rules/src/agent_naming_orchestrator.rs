// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use async_trait::async_trait;
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{ErrorMessage, FilePath, ScanError};

use shared::common::{LayerMapVO, PatternList};
use shared::config_system::ArchitectureConfig;
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

        let all_files = shared::naming_rules::utility_naming_filesystem::walk_recursive(
            target,
            Some(&self.ignored_patterns),
        );
        let files = shared::naming_rules::utility_file_filter::filter_source_files(&all_files);

        let mut naming_results = LintResultList::new(Vec::new());
        let mut suffix_results = LintResultList::new(Vec::new());

        let ((), ()) = tokio::join!(
            self.deps.naming_convention_checker.check_file_naming(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                &files,
                target,
                &mut naming_results,
            ),
            self.deps.suffix_prefix_checker.check_domain_suffixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                &files,
                target,
                &mut suffix_results,
            ),
        );

        naming_results.values.extend(suffix_results.values);
        Ok(naming_results.values)
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl NamingOrchestrator {
    pub fn new(deps: NamingOrchestratorDeps) -> Self {
        let mut values: Vec<String> = deps
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
            .collect();
        // Default-excluded directories are skipped even without config `ignored_paths`.
        // `tests` must never be linted (test scaffolding is not production code).
        if !values.iter().any(|v| v == "tests") {
            values.push("tests".to_string());
        }
        let ignored_patterns = PatternList { values };
        Self {
            deps,
            ignored_patterns,
        }
    }
}
