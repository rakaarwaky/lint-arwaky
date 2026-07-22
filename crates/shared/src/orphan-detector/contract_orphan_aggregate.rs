// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

/// Aggregate that detects orphan (unreferenced) files in a project.
///
/// AES308 requires that every source file be reachable from at least one
/// entry point. This aggregate builds a dependency graph, identifies
/// orphan entry points, and reports violations.
pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult>;
}
