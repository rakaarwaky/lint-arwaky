// PURPOSE: IOrphanAggregate — aggregate trait bundling all orphan detection protocols
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::common::taxonomy_path_vo::FilePath;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[FilePath]) -> HashSet<FilePath>;
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;
}
