// PURPOSE: Shared utilities for Rust external linter adapters
use shared::common::taxonomy_path_vo::FilePath;
use std::path::Path;

pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }

    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.toml").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").exists() {
            return match FilePath::new(parent.to_string_lossy().replace('\\', "/")) {
                Ok(fp) => fp,
                Err(_) => path.clone(),
            };
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").exists() {
                return match FilePath::new(grandparent.to_string_lossy().replace('\\', "/")) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                };
            }
        }
    }

    FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
}

pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }

    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.lock").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").exists() {
            return match FilePath::new(parent.to_string_lossy().replace('\\', "/")) {
                Ok(fp) => fp,
                Err(_) => path.clone(),
            };
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").exists() {
                return match FilePath::new(grandparent.to_string_lossy().replace('\\', "/")) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                };
            }
        }
    }

    FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
}
