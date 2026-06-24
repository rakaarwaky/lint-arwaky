// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = match Path::new(filename).file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => "",
    };

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
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}
