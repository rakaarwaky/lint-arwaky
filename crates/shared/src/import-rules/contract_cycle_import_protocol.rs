// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → CycleImportAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    /// Scan all files for circular dependency cycles (AES205).
    fn scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;

    /// Adapter: converts ICycleImportProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    async fn check_cycles(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    /// Detect cycle edges in a directed graph using DFS 3-coloring.
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    /// Normalize a file/module name to its architectural layer name.
    fn normalize_to_layer(&self, name: &str) -> LayerNameVO;
}
