// PURPOSE: LayerDetectionAnalyzer — layer detection via filename prefix (FRD v1.1)
// Delegates to utility functions for all logic; this struct is a thin wrapper
// that implements IAnalyzer and ILayerDetectionAggregate traits for DI.

use std::collections::HashMap;
use std::path::Path;

use shared::common::contract_parser_protocol::ISourceParserProtocol;
use shared::common::contract_system_protocol::IFileSystemProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_path_helper;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::LayerNameVO;
use std::sync::Arc;

pub struct LayerDetectionAnalyzer {
    pub config: ArchitectureConfig,
    pub layer_map: LayerMapVO,
    pub fs: Arc<dyn IFileSystemProtocol>,
    pub parser: Arc<dyn ISourceParserProtocol>,
}

impl LayerDetectionAnalyzer {
    /// Construct using utility_config_merger for config merging.
    pub fn new(
        config: ArchitectureConfig,
        fs: Arc<dyn IFileSystemProtocol>,
        parser: Arc<dyn ISourceParserProtocol>,
    ) -> Self {
        let (merged_layers, _) = shared::config_system::utility_config_merger::merge_config(&config);
        let mut config = config;
        config.layers = merged_layers;
        let layer_map = LayerMapVO::new(config.layers.clone());
        Self {
            config,
            layer_map,
            fs,
            parser,
        }
    }

    /// Detect layer from filename — delegates to utility_layer_detector.
    pub fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .map_or("", |s| s);

        let layer = shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let layer_keys: Vec<String> = self.config.layers.keys().map(|k| k.to_string()).collect();
        Some(shared::common::utility_layer_detector::resolve_specialized_layer(
            &layer,
            file_path,
            &layer_keys,
        ))
    }

    /// Detect module layer — delegates to utility_layer_detector.
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        let layer_names: Vec<String> = self.config.layers.keys().map(|k| k.to_string()).collect();
        shared::common::utility_layer_detector::detect_module_layer(module, &layer_names)
    }

    /// Look up a LayerDefinition by layer name string.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config
            .layers
            .get(&LayerNameVO::new(layer))
            .or_else(|| {
                let base = match layer.split('(').next() {
                    Some(s) => s,
                    None => layer,
                };
                self.config.layers.get(&LayerNameVO::new(base))
            })
    }
}

impl shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
    for LayerDetectionAnalyzer
{
    fn config(&self) -> &ArchitectureConfig {
        &self.config
    }
    fn layer_map(&self) -> &LayerMapVO {
        &self.layer_map
    }
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO> {
        self.detect_layer(&f.value, &root_dir.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
}

impl IAnalyzer for LayerDetectionAnalyzer {
    fn fs(&self) -> &dyn IFileSystemProtocol {
        &*self.fs
    }
    fn parser(&self) -> &dyn ISourceParserProtocol {
        &*self.parser
    }
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO> {
        self.detect_module_layer(&module_path.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
}

impl shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate
    for LayerDetectionAnalyzer
{
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        self.detect_layer(file_path, root_dir)
    }

    fn get_layer_def(
        &self,
        layer: &str,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer).cloned()
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![
            "_container.rs".to_string(),
            "_container.py".to_string(),
            "_container.ts".to_string(),
            "_container.js".to_string(),
            "_entry.rs".to_string(),
            "_entry.py".to_string(),
            "_entry.ts".to_string(),
            "_entry.js".to_string(),
            "main.rs".to_string(),
            "lib.rs".to_string(),
            "main.py".to_string(),
            "main.ts".to_string(),
            "main.js".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
        ]
    }

    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        &self.config
    }
}
