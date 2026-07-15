// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → CycleImportAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use async_trait::async_trait;

/// Unified protocol for cycle import detection (AES205).
/// Combines scanning logic and core cycle detection algorithm.
#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    /// Scan all files for circular dependency cycles (AES205).
    fn scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[String],
        root_dir: &str,
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
    fn normalize_to_layer(&self, name: &str) -> String;

    /// Returns the inner FilePath if result is Ok, otherwise returns FilePath::default().
    fn filepath_or_default(&self, result: Result<FilePath, String>) -> FilePath;

    /// Scan all files for circular dependency cycles (AES205) — internal implementation.
    fn do_scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;

    /// Detect cycle edges in a directed graph using DFS 3-coloring — internal implementation.
    fn do_detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    /// Normalize a file/module name to its architectural layer name — internal implementation.
    fn do_normalize_to_layer(&self, name: &str) -> String;

    /// DFS 3-coloring traversal to detect back-edges (cycles).
    fn dfs_3color(
        &self,
        node: &str,
        graph: &std::collections::HashMap<String, Vec<String>>,
        color: &mut std::collections::HashMap<
            String,
            crate::import_rules::taxonomy_cycle_color_vo::Color,
        >,
        parent: &mut std::collections::HashMap<String, String>,
        cycle_edges: &mut std::collections::HashSet<(String, String)>,
    );

    /// Extract cycle nodes from source to target using parent tracking.
    fn extract_cycle_nodes(
        &self,
        src: &str,
        tgt: &str,
        parent: &std::collections::HashMap<String, String>,
    ) -> Option<Vec<String>>;
}
