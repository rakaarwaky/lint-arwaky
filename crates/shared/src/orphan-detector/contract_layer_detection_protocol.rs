use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub struct LayerDetectionResult {
    pub layer_name: String,
    pub definition: Option<crate::common::taxonomy_definition_vo::LayerDefinition>,
}

pub trait ILayerDetectionProtocol: Send + Sync {
    fn detect_layer(
        &self,
        file: &FilePath,
        config: &ArchitectureConfig,
    ) -> Option<LayerDetectionResult>;
}
