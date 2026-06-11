// PURPOSE: IOrphanAggregate — aggregate trait bundling all orphan detection protocols
use code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use output_report::taxonomy_result_vo::LintResult;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}
