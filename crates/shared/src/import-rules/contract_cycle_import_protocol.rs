// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → CycleImportAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use async_trait::async_trait;

/// Unified protocol for cycle import detection (AES205).
/// Combines scanning logic and core cycle detection algorithm.
#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    /// Scan all files for circular dependency cycles (AES205).
    fn scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;

    /// Adapter: converts ICycleImportProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    /// Detect cycle edges in a directed graph using DFS 3-coloring.
    fn pure_detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    /// Normalize a file/module name to its architectural layer name.
    fn pure_normalize_to_layer(&self, name: &str) -> String;
}
