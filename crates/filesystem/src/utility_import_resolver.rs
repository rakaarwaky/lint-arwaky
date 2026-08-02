// Import path resolution helper
// Utility: stateless standalone functions

use std::path::{Path, PathBuf};

use crate::utility_filesystem_io::is_file;

/// Normalize a module path for filesystem lookup.
pub fn normalize_module_path(module_path: &str) -> String {
    module_path.replace('-', "_")
}

/// Resolve a relative import path to an absolute file path.
pub fn resolve_import_path(
    source_file: &Path,
    import_path: &str,
    root: &Path,
) -> Option<PathBuf> {
    // Handle crate:: and super:: prefixes
    if import_path.starts_with("crate::") || import_path.starts_with("super::") {
        return resolve_crate_path(source_file, import_path, root);
    }

    // Handle relative paths
    if import_path.starts_with('.') {
        return resolve_relative_path(source_file, import_path, root);
    }

    // Handle absolute module paths
    resolve_absolute_path(import_path, root)
}

fn resolve_crate_path(source_file: &Path, import_path: &str, root: &Path) -> Option<PathBuf> {
    let stripped = import_path
        .strip_prefix("crate::")
        .unwrap_or(import_path);
    let parts: Vec<&str> = stripped.split("::").collect();

    // Find the crate root from source file
    let mut current = source_file.parent()?;
    while current != root {
        if current.join("src").is_dir() || current.join("lib.rs").is_dir() {
            break;
        }
        current = current.parent()?;
    }

    let mut path = current.join("src");
    for part in &parts {
        path = path.join(part);
    }

    // Try file extensions
    for ext in &[".rs", ".py", ".ts", ".js"] {
        let candidate = path.with_extension(ext);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    // Try as directory with mod.rs/index.ts/index.js/__init__.py
    for mod_file in &["mod.rs", "index.ts", "index.js", "__init__.py"] {
        let candidate = path.join(mod_file);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    None
}

fn resolve_relative_path(source_file: &Path, import_path: &str, _root: &Path) -> Option<PathBuf> {
    let base = source_file.parent()?;
    let mut path = base.to_path_buf();

    for part in import_path.split('/') {
        match part {
            "." => {}
            ".." => {
                path.pop();
            }
            _ => path = path.join(part),
        }
    }

    // Try file extensions
    for ext in &[".rs", ".py", ".ts", ".js", ".tsx", ".jsx"] {
        let candidate = path.with_extension(ext);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    // Try as directory with index/mod files
    for mod_file in &["mod.rs", "index.ts", "index.js", "__init__.py"] {
        let candidate = path.join(mod_file);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    None
}

fn resolve_absolute_path(module_path: &str, root: &Path) -> Option<PathBuf> {
    let parts: Vec<&str> = module_path.split("::").collect();
    let mut path = root.to_path_buf();

    for part in &parts {
        path = path.join(part);
    }

    // Try file extensions
    for ext in &[".rs", ".py", ".ts", ".js"] {
        let candidate = path.with_extension(ext);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    // Try as directory
    for mod_file in &["mod.rs", "index.ts", "index.js", "__init__.py"] {
        let candidate = path.join(mod_file);
        if is_file(&candidate) {
            return Some(candidate);
        }
    }

    None
}

/// Resolve barrel re-exports to the original source file.
pub fn resolve_barrel_reexport(
    barrel_file: &Path,
    imported_name: &str,
    root: &Path,
) -> Option<PathBuf> {
    let content = crate::utility_filesystem_io::read_to_string(barrel_file).ok()?;

    // Look for pub use statements
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("pub use") && line.contains(imported_name) {
            // Extract the path from pub use path::name;
            if let Some(path_part) = line
                .strip_prefix("pub use ")
                .map(|s| s.trim_end_matches(';').trim())
            {
                let parts: Vec<&str> = path_part.split("::").collect();
                if let Some(last) = parts.last() {
                    if *last == imported_name {
                        let module_path = parts[..parts.len() - 1].join("::");
                        return resolve_absolute_path(&module_path, root);
                    }
                }
            }
        }
    }

    None
}