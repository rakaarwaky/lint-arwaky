// PURPOSE: DefaultLayerDetector — ILayerDetectionProtocol stub for DI fallback
//
// Null object pattern: returns defaults for all methods.
// Used when no real layer detection analyzer is provided.
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct DefaultLayerDetector;

// ─── Block 2: Public Contract ─────────────────────────────
impl ILayerDetectionProtocol for DefaultLayerDetector {
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }

    fn detect_layer(&self, _file_path: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
        None
    }

    fn get_layer_def(
        &self,
        _layer: &LayerNameVO,
    ) -> Option<shared::taxonomy_definition_vo::LayerDefinition> {
        None
    }

    fn get_orphan_entry_points(&self) -> Vec<FilePath> {
        Vec::new()
    }

    fn extract_layer_from_prefix(&self, _filename: &FilePath) -> Option<LayerNameVO> {
        None
    }

    fn resolve_specialized_layer(
        &self,
        base_layer: &LayerNameVO,
        _file_path: &FilePath,
    ) -> LayerNameVO {
        base_layer.clone()
    }

    fn detect_module_layer(&self, _module: &str) -> Option<LayerNameVO> {
        None
    }

    fn refine_module_layer(&self, base_name: &LayerNameVO, _parts: &[&str]) -> LayerNameVO {
        base_name.clone()
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl DefaultLayerDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultLayerDetector {
    fn default() -> Self {
        Self::new()
    }
}
