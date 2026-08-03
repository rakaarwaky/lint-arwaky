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
    // String-based normalization — avoids std::fs::canonicalize() I/O
    let normalized_file = normalize_path(file_path);
    let normalized_root = normalize_path(root_dir).trim_end_matches('/').to_string();

    let file_path = Path::new(&normalized_file);
    let root_path = Path::new(&normalized_root);

    match file_path.strip_prefix(root_path) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => {
            // Fallback: try string-based prefix removal
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

/// Normalize a path string without filesystem I/O.
/// Collapses `.`, `..`, and redundant separators.
fn normalize_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let mut components: Vec<&str> = Vec::new();
    for part in normalized.split('/') {
        match part {
            "" | "." => continue,
            ".." => {
                components.pop();
            }
            other => components.push(other),
        }
    }
    format!("/{}", components.join("/"))
}
