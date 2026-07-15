// PURPOSE: ICycleAnalyzerPort — contract trait for cycle detection
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;

pub trait ICycleAnalyzerPort: Send + Sync {
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;
}
