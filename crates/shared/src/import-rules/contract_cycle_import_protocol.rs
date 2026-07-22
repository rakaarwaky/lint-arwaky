// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_name_vo::SymbolName;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;

    async fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &crate::common::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;
    fn normalize_to_layer(&self, name: &str) -> LayerNameVO;
}
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
