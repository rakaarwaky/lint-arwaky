// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → CycleImportAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    fn scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;

    async fn check_cycles(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    fn normalize_to_layer(&self, name: &str) -> LayerNameVO;

    fn filepath_or_default(&self, result: Result<FilePath, ErrorMessage>) -> FilePath;

    fn do_scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;

    fn do_detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    fn do_normalize_to_layer(&self, name: &str) -> LayerNameVO;

    fn dfs_3color(
        &self,
        node: &LayerNameVO,
        graph: &std::collections::HashMap<LayerNameVO, Vec<LayerNameVO>>,
        color: &mut std::collections::HashMap<
            LayerNameVO,
            crate::import_rules::taxonomy_cycle_color_vo::Color,
        >,
        parent: &mut std::collections::HashMap<LayerNameVO, LayerNameVO>,
        cycle_edges: &mut std::collections::HashSet<(LayerNameVO, LayerNameVO)>,
    );

    fn extract_cycle_nodes(
        &self,
        src: &LayerNameVO,
        tgt: &LayerNameVO,
        parent: &std::collections::HashMap<LayerNameVO, LayerNameVO>,
    ) -> Option<Vec<LayerNameVO>>;
}
