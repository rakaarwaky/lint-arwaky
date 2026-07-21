// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_constant::SOURCE_EXTENSIONS;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct NamingOrchestrator {
    naming_convention_checker: Arc<dyn INamingConventionChecker>,
    suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    config: Arc<ArchitectureConfig>,
    layer_map: Arc<LayerMapVO>,
    ignored_patterns: PatternList,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let all_files = shared::naming_rules::utility_naming_filesystem::walk_recursive(
            target,
            Some(&self.ignored_patterns),
        );
        let files = Self::filter_source_files(&all_files);

        self.naming_convention_checker
            .check_file_naming(
                self.config.as_ref(),
                self.layer_map.as_ref(),
                &files,
                target,
                &mut results,
            )
            .await;
        self.suffix_prefix_checker
            .check_domain_suffixes(
                self.config.as_ref(),
                self.layer_map.as_ref(),
                &files,
                target,
                &mut results,
            )
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl NamingOrchestrator {
    pub fn new(
        naming_convention_checker: Arc<dyn INamingConventionChecker>,
        suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
        config: Arc<ArchitectureConfig>,
        layer_map: Arc<LayerMapVO>,
    ) -> Self {
        let ignored_patterns = PatternList {
            values: config
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
            naming_convention_checker,
            suffix_prefix_checker,
            config,
            layer_map,
            ignored_patterns,
        }
    }

    fn filter_source_files(
        files: &shared::common::taxonomy_paths_vo::FilePathList,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        let filtered: Vec<FilePath> = files
            .values
            .iter()
            .filter(|f| {
                let path = Path::new(&f.value);
                path.extension()
                    .and_then(|e| e.to_str())
                    .map(|ext| SOURCE_EXTENSIONS.contains(&ext))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        shared::common::taxonomy_paths_vo::FilePathList::new(filtered)
    }
}
