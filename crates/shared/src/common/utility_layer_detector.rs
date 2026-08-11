// PURPOSE: Layer detection utility — pure function, simple prefix check
use std::collections::HashMap;
use std::path::Path;

use crate::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use crate::common::taxonomy_layer_vo::LayerNameVO;

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

    // Strategy 1: Direct match with layer names.
    if let Some(layer) = direct_layer_match(&meaningful_parts, layer_names) {
        return Some(layer);
    }

    // Strategy 2: Prefix-based match on segments.
    if let Some(layer) = prefix_segment_match(&meaningful_parts) {
        return Some(layer);
    }

    // Strategy 3 & 4: Last-part strategies.
    if let Some(last_part) = meaningful_parts.last() {
        if let Some(layer) = rust_module_strategy(last_part) {
            return Some(layer);
        }
        if let Some(layer) = python_js_path_strategy(&meaningful_parts) {
            return Some(layer);
        }
    }

    None
}

/// Strategy 1: Direct segment match with layer base names.
fn direct_layer_match(meaningful_parts: &[&str], layer_names: &[String]) -> Option<String> {
    for name in layer_names {
        let base_name = name.split('(').next().unwrap_or(name);
        if meaningful_parts.contains(&base_name) {
            return Some(base_name.to_string());
        }
    }
    None
}

/// Strategy 2: Prefix-based match on segments.
fn prefix_segment_match(meaningful_parts: &[&str]) -> Option<String> {
    for part in meaningful_parts {
        if let Some(layer) = detect_layer_from_prefix(part) {
            return Some(layer);
        }
    }
    None
}

/// Strategy 3: Handle Rust module paths with "::" separators.
fn rust_module_strategy(last_part: &str) -> Option<String> {
    if last_part.contains("::") {
        if let Some(stem) = last_part.rsplit("::").next() {
            if let Some(layer) = detect_layer_from_prefix(stem) {
                return Some(layer);
            }
        }
    }
    None
}

/// Strategy 4: Scan all meaningful parts for layer prefixes.
fn python_js_path_strategy(meaningful_parts: &[&str]) -> Option<String> {
    for part in meaningful_parts {
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
    let scan_dir = build_scan_dir(root_dir, module_path);
    scan_dir_for_layer(&scan_dir)
}

/// Build the filesystem path from root_dir and dotted module_path.
fn build_scan_dir(root_dir: &str, module_path: &str) -> String {
    let dir_path = root_dir
        .trim_end_matches(std::path::MAIN_SEPARATOR)
        .trim_end_matches('/');
    let rel_path = module_path.replace('.', std::path::MAIN_SEPARATOR_STR);
    format!("{}/{}", dir_path, rel_path)
}

/// Scan a directory for layer-prefixed files and return the first match.
fn scan_dir_for_layer(scan_dir: &str) -> Option<String> {
    let Ok(read_dir) = std::fs::read_dir(scan_dir) else {
        return None;
    };
    for entry in read_dir.flatten() {
        if let Some(layer) = check_entry_for_layer(&entry) {
            return Some(layer);
        }
    }
    None
}

/// Check a directory entry for layer prefix. Returns layer if found.
fn check_entry_for_layer(entry: &std::fs::DirEntry) -> Option<String> {
    let entry_path = entry.path();
    if !entry_path.is_file() {
        return None;
    }
    let filename = entry_path.file_name().and_then(|n| n.to_str())?;
    detect_layer_from_prefix(filename)
}
