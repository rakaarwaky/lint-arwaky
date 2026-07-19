// PURPOSE: ILayerDetectionAggregate — contract trait for layer detection (detect_layer + get_layer_def)
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

/// Slim aggregate for layer detection — used by orphan detector and orchestrator.
/// Container implements this; orchestrator calls individual checker protocols directly.
pub trait ILayerDetectionAggregate: Send + Sync {
    fn detect_layer(&self, file_path: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
    fn get_layer_def(&self, layer: &LayerNameVO) -> Option<LayerDefinition>;
    fn get_orphan_entry_points(&self) -> Vec<FilePath>;
    fn config(&self) -> &ArchitectureConfig;
}
