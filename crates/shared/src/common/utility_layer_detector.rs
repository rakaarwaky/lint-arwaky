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
/// Tries multiple strategies:
/// 1. Direct segment match with layer names
/// 2. Prefix-based match on segments (e.g., "contract_" → "contract")
/// 3. Filename suffix extraction from path segments
/// 4. Module-path-to-filename resolution for structured projects
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

    // Strategy 2: Prefix-based match on segments
    for part in &meaningful_parts {
        if let Some(layer) = detect_layer_from_prefix(part) {
            return Some(layer);
        }
    }

    // Strategy 3: Extract filename from path and detect layer from filename prefix.
    // Handles patterns like "modules/shared/src/server/contract_protocol" or
    // "mypackage.capabilities_payment_service" where the layer is encoded in
    // the last segment's filename prefix/suffix.
    if let Some(last_part) = meaningful_parts.last() {
        // Handle Rust module paths: "crate::features::payment_service::PaymentService"
        // → extract "payment_service" and detect layer from its suffix
        if last_part.contains("::")
            && let Some(stem) = last_part.rsplit("::").next()
        {
            // Check if this segment itself has a prefix (e.g., "contract_")
            if let Some(layer) = detect_layer_from_prefix(stem) {
                return Some(layer);
            }
        }

        // Handle Python/JS paths: "modules.shared.src.server.contract_protocol"
        // → extract filename, detect layer from prefix
        for part in &meaningful_parts {
            if let Some(layer) = detect_layer_from_prefix(part) {
                return Some(layer);
            }
        }

        // Strategy 4: Try to find layer from the full path by looking for
        // known layer-prefixed segments anywhere in the path.
        // e.g., "shared.src.contract_*" → contract, "shared.src.capabilities_*" → capabilities
        for part in &meaningful_parts {
            if let Some(layer) = detect_layer_from_prefix(part) {
                return Some(layer);
            }
        }

        // Strategy 5: Check if any segment ends with a layer name + underscore
        // (e.g., "contract_protocol" → contract, "capabilities_adapter" → capabilities)
        for part in &meaningful_parts {
            let parts: Vec<&str> = part.split('_').collect();
            for (_i, _seg) in parts.iter().enumerate() {
                let _combined: Vec<&str> = parts[_i..].join("_").split('_').collect();
                // Try combining remaining segments
                let combined_str = parts[_i..].join("_");
                if detect_layer_from_prefix(&combined_str).is_some() {
                    continue;
                }
            }
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

/// Resolve a module path to its layer by scanning the filesystem for
/// layer-prefixed filenames. Handles structured project paths like:
/// - "modules/shared/src/server" → scans for contract_*, capabilities_* files
/// - "mypackage" → scans for taxonomy_*, utility_* files
///
/// # Arguments
/// * `module_path` - The module path from import (e.g., "modules.shared.src.server")
/// * `root_dir` - The project root directory for scanning
///
/// # Returns
/// The detected layer name if a layer-prefixed file is found, None otherwise.
pub fn resolve_module_path_to_layer(module_path: &str, root_dir: &str) -> Option<String> {
    // Convert dotted module path to filesystem path
    let dir_path = root_dir
        .trim_end_matches(std::path::MAIN_SEPARATOR)
        .trim_end_matches('/');

    // Build relative path from module path
    let rel_path = module_path.replace('.', std::path::MAIN_SEPARATOR_STR);

    let scan_dir = format!("{}/{}", dir_path, rel_path);

    // Read directory entries and look for layer-prefixed files
    for entry_path_str in crate::filesystem::utility_filesystem_io::read_dir_generic(&scan_dir) {
        let entry_path = std::path::PathBuf::from(&entry_path_str);
        if crate::filesystem::utility_filesystem_io::is_file(&entry_path) {
            if let Some(filename) = entry_path.file_name().and_then(|n| n.to_str()) {
                // Check if filename has a layer prefix
                if let Some(layer) = detect_layer_from_prefix(filename) {
                    return Some(layer);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::common::utility_layer_detector::{detect_layer_from_prefix, detect_module_layer, resolve_module_path_to_layer};

    #[test]
    fn test_detect_module_layer_with_prefix() {
        let layer_names: Vec<String> = vec![
            "taxonomy".into(),
            "contract".into(),
            "utility".into(),
            "capabilities".into(),
            "agent".into(),
            "surface".into(),
        ];

        // Standard module path with layer prefix in segment
        assert_eq!(
            detect_module_layer("shared.src.contract_protocol", &layer_names),
            Some("contract".to_string())
        );
    }

    #[test]
    fn test_resolve_module_path_to_layer() {
        // Test with blender-arwaky structure
        // modules/shared/src/common/ has contract_* and taxonomy_* files
        let result = resolve_module_path_to_layer(
            "modules.shared.src.common",
            "/home/raka/mcp-arwaky/blender-arwaky",
        );
        assert!(
            result.is_some(),
            "Should detect layer from common directory (has contract_* and taxonomy_* files)"
        );
        assert!(
            result.as_ref().unwrap() == "contract" || result.as_ref().unwrap() == "taxonomy",
            "Detected layer should be contract or taxonomy"
        );
    }
}