// PURPOSE: IOrphanGraphResolverProtocol — contract trait for building orphan analysis graph context
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;

pub trait IOrphanGraphResolverProtocol: Send + Sync {
    fn build_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_entry_points(&self, files: &[String], configured: &[String]) -> Vec<String>;
}
