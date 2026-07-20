// PURPOSE: Layer detection utility — pure function, simple prefix check
use std::collections::HashMap;
use std::path::Path;

use crate::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use crate::taxonomy_layer_vo::LayerNameVO;

/// Detect architectural layer from filename prefix.
///
/// Returns the layer name if the filename starts with a valid prefix, otherwise None.
///
/// # Examples
/// - "taxonomy_foo.rs" → Some("taxonomy")
/// - "contract_bar.rs" → Some("contract")
/// - "foo.rs" → None
pub fn detect_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
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

/// Resolve specialised sub-layer from file suffix.
///
/// E.g., "capabilities_command" with base_layer="capabilities":
///   → suffix = "command"
///   → checks if "capabilities(command)" exists in config
///   → returns "capabilities(command)" if found, else "capabilities"
pub fn resolve_specialized_layer(
    base_layer: &str,
    file_path: &str,
    layer_keys: &[String],
) -> String {
    let basename = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if let Some(underscore_pos) = basename.rfind('_') {
        let suffix = &basename[underscore_pos + 1..];
        if !suffix.is_empty() {
            let specialized = format!("{}({})", base_layer, suffix);
            if layer_keys.contains(&specialized) {
                return specialized;
            }
        }
    }

    base_layer.to_string()
}

/// Detect layer from module path (from import statement).
///
/// Tries 3 strategies:
/// 1. Direct segment match (e.g., "shared::taxonomy::..." → "taxonomy")
/// 2. Prefix-based match (e.g., "taxonomy_definition_vo" → "taxonomy")
pub fn detect_module_layer(module: &str, layer_names: &[String]) -> Option<String> {
    let meaningful_parts: Vec<&str> = module
        .split([':', '.', '/', '\\'])
        .filter(|p| !p.is_empty())
        .collect();

    if meaningful_parts.is_empty() {
        return None;
    }

    // Strategy 1: Direct match with layer names
    for name in layer_names {
        let base_name = match name.split('(').next() {
            Some(s) => s,
            None => name,
        };
        if meaningful_parts.contains(&base_name) {
            return Some(base_name.to_string());
        }
    }

    // Strategy 2: Prefix-based match
    for part in &meaningful_parts {
        if let Some(layer) = detect_layer_from_prefix(part) {
            return Some(layer);
        }
    }

    None
}

/// Extract filename from file path.
///
/// Returns the filename (last component) as a string slice, or empty string if extraction fails.
pub fn extract_filename(file_path: &str) -> &str {
    Path::new(file_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Collect layer keys as strings from a LayerMapVO.
pub fn collect_layer_keys(layer_map: &LayerMapVO) -> Vec<String> {
    layer_map.values.keys().map(|k| k.to_string()).collect()
}

/// Look up a LayerDefinition by layer name string.
///
/// Tries direct lookup first, then falls back to base name (before parenthesis).
pub fn get_layer_def<'a>(
    layer: &str,
    layers: &'a HashMap<LayerNameVO, LayerDefinition>,
) -> Option<&'a LayerDefinition> {
    layers.get(&LayerNameVO::new(layer)).or_else(|| {
        let base = match layer.split('(').next() {
            Some(s) => s,
            None => layer,
        };
        layers.get(&LayerNameVO::new(base))
    })
}
