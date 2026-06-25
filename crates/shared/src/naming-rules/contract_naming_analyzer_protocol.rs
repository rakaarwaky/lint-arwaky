// PURPOSE: INamingAnalyzerProtocol — protocol trait for naming-rules analyzer dependency isolation
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::LayerNameVO;

pub trait INamingAnalyzerProtocol: Send + Sync {
    fn config(&self) -> &ArchitectureConfig;
    fn layer_map(&self) -> &LayerMapVO;
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
}
