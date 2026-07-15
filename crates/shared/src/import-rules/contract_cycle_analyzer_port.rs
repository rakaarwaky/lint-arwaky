// PURPOSE: ICycleAnalyzerPort — contract trait for cycle detection
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;

pub trait ICycleAnalyzerPort: Send + Sync {
    /// Detect cycle edges in a directed graph using DFS 3-coloring.
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    /// Normalize a file/module name to its architectural layer name.
    fn pure_normalize_to_layer(&self, name: &str) -> String;
}
