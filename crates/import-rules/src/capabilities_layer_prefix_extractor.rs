// PURPOSE: LayerPrefixExtractor — capabilities layer for layer prefix extraction
use shared::import_rules::contract_layer_prefix_port::ILayerPrefixPort;
use std::path::Path;

pub struct LayerPrefixExtractor;

impl LayerPrefixExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LayerPrefixExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl ILayerPrefixPort for LayerPrefixExtractor {
    fn extract_layer_from_prefix(&self, filename: &str) -> Option<String> {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];

        for &(prefix, layer) in PREFIX_MAP {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }
}
