// PURPOSE: ICycleAnalysisProtocol — contract trait for circular dependency detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → DependencyCycleAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use async_trait::async_trait;

/// Abstract protocol for circular dependency (cycle) detection.
/// Implemented by capabilities layer in the code-analysis crate.
#[async_trait]
pub trait ICycleAnalysisProtocol: Send + Sync {
    /// Scan all files for circular dependency cycles (AES205).
    fn scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;

    /// Adapter: converts ICycleAnalysisProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
