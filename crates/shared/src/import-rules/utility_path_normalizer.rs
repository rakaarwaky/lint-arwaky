// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
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

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = std::fs::canonicalize(file_path).unwrap_or_else(|_| std::path::PathBuf::from(file_path))
        .to_string_lossy()
        .replace('\\', "/");
    let normalized_root = std::fs::canonicalize(root_dir).unwrap_or_else(|_| std::path::PathBuf::from(root_dir))
        .to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_string();

    let file_path = Path::new(&normalized_file);
    let root_path = Path::new(&normalized_root);

    match file_path.strip_prefix(root_path) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => {
            // Fallback: try string-based prefix removal
            // Ensure root ends with / for proper prefix matching
            let root_prefix = if normalized_root.ends_with('/') {
                normalized_root.clone()
            } else {
                format!("{}/", normalized_root)
            };

            if normalized_file.starts_with(&root_prefix) {
                normalized_file[root_prefix.len()..].to_string()
            } else if normalized_file == normalized_root {
                String::new()
            } else {
                normalized_file
            }
        }
    }
}
