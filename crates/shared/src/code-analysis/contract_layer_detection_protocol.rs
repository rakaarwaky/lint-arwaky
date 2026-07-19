// PURPOSE: ILayerDetectionProtocol — contract trait for layer detection (all methods)
use crate::code_analysis::taxonomy_bypass_utility;
use crate::code_analysis::taxonomy_duplication_utility;
use crate::code_analysis::taxonomy_import_source_vo;
use crate::code_analysis::taxonomy_target_utility;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub trait ILayerDetectionProtocol: Send + Sync {
    fn detect_layer(&self, file_path: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
    fn get_layer_def(&self, layer: &LayerNameVO) -> Option<LayerDefinition>;
    fn get_orphan_entry_points(&self) -> Vec<FilePath>;
    fn config(&self) -> &ArchitectureConfig;
    fn extract_layer_from_prefix(&self, filename: &FilePath) -> Option<LayerNameVO>;
    fn resolve_specialized_layer(
        &self,
        base_layer: &LayerNameVO,
        file_path: &FilePath,
    ) -> LayerNameVO;
    fn detect_module_layer(&self, module: &str) -> Option<LayerNameVO>;
    fn refine_module_layer(&self, base_name: &LayerNameVO, parts: &[&str]) -> LayerNameVO;
}
