use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_layer_detection_protocol::{
    ILayerDetectionProtocol, LayerDetectionResult,
};

pub struct CapabilitiesLayerDetector;

impl ILayerDetectionProtocol for CapabilitiesLayerDetector {
    fn detect_layer(
        &self,
        file: &FilePath,
        config: &ArchitectureConfig,
    ) -> Option<LayerDetectionResult> {
        let filename = shared::common::utility_layer_detector::extract_filename(file.value());
        let base_layer = shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let layer_keys: Vec<String> = config.layers.keys().map(|k| k.value.to_string()).collect();
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer,
            file.value(),
            &layer_keys,
        );
        let definition =
            shared::common::utility_layer_detector::get_layer_def(&layer_str, &config.layers);
        Some(LayerDetectionResult {
            layer_name: layer_str,
            definition: definition.cloned(),
        })
    }
}
