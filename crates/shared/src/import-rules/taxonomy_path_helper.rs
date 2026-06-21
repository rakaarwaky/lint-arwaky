// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

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

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.replace('\\', "/"));
    let normalized_root = Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| root_dir.trim_end_matches('/').replace('\\', "/"));
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}

pub fn match_layer_recursive(rel: &str, path_def: &str) -> bool {
    let last_segment = path_def.rsplit('/').next().unwrap_or(path_def);
    rel.starts_with(path_def) || rel.starts_with(last_segment)
}

pub fn match_layer_nonrecursive(rel: &str, path_def: &str) -> bool {
    let norm_path_def = path_def.trim_end_matches('/');

    let parent_dir = match Path::new(rel).parent().and_then(|p| p.to_str()) {
        Some("") => ".",
        Some(p) => p.trim_end_matches('/'),
        None => ".",
    };

    if parent_dir == norm_path_def {
        return true;
    }

    if parent_dir == "." && !norm_path_def.is_empty() && norm_path_def != "." {
        return true;
    }

    if parent_dir == "." && rel.ends_with(norm_path_def) {
        return true;
    }

    if parent_dir == "." && !norm_path_def.is_empty() {
        return true;
    }

    false
}
