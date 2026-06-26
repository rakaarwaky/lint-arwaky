// PURPOSE: Shared utilities for Rust external linter adapters
//
// Provides two walk-up directory resolvers:
//   - resolve_cargo_working_dir: finds parent dir with Cargo.toml (for cargo fmt, cargo clippy)
//   - resolve_cargo_lock_working_dir: finds parent dir with Cargo.lock (for cargo-audit)
//
// Both walk up to 2 levels from the given path. If no Cargo.toml/lock is found,
// a sentinel path containing "nonexistent_directory_for_cargo_*" is returned so
// callers can detect and skip gracefully.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_cargo_working_dir_empty() {
        let path = FilePath::new("".to_string()).unwrap_or_default();
        let result = resolve_cargo_working_dir(&path);
        assert!(result.value.is_empty());
    }

    #[test]
    fn test_resolve_cargo_working_dir_with_cargo_toml() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_rs_cargo");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();
        let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        let result = resolve_cargo_working_dir(&path);
        assert_eq!(result.value, dir.to_string_lossy().to_string());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_resolve_cargo_working_dir_file_in_cargo_project() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_rs_file");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();
        std::fs::write(dir.join("main.rs"), "fn main() {}").unwrap();
        let file_path = dir.join("main.rs");
        let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap_or_default();
        let result = resolve_cargo_working_dir(&path);
        assert_eq!(result.value, dir.to_string_lossy().to_string());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_resolve_cargo_working_dir_no_cargo_toml() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_rs_none");
        let _ = std::fs::create_dir_all(&dir);
        let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        let result = resolve_cargo_working_dir(&path);
        assert!(result.value.contains("nonexistent"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_resolve_cargo_lock_working_dir_empty() {
        let path = FilePath::new("".to_string()).unwrap_or_default();
        let result = resolve_cargo_lock_working_dir(&path);
        assert!(result.value.is_empty());
    }

    #[test]
    fn test_resolve_cargo_lock_working_dir_with_lock() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_rs_lock");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("Cargo.lock"), "version = 3\n").unwrap();
        let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        let result = resolve_cargo_lock_working_dir(&path);
        assert_eq!(result.value, dir.to_string_lossy().to_string());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
