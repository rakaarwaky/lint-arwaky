// PURPOSE: Workspace utility — locate workspace root and verify container wiring without dependency injection
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_detector::utility_file_cache;

/// Walk parent directories from `start` to locate the workspace root:
/// a directory that holds a member dir (crates/packages/modules) AND a
/// manifest (Cargo.toml / package.json / pyproject.toml).
pub fn find_workspace_root(start: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut current = start.to_path_buf();
    loop {
        let has_cargo = current.join("Cargo.toml").exists();
        let has_package_json = current.join("package.json").exists();
        let has_pyproject = current.join("pyproject.toml").exists();
        let has_member_dir = member_dirs.iter().any(|d| current.join(d).is_dir());

        if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
            return Ok(current);
        }

        if !current.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace root not found",
            ));
        }
    }
}

/// Returns true if any container/entry file under the workspace root references
/// one of `identifiers`.
pub fn check_wired_in_container(
    workspace_root: &std::path::Path,
    identifiers: &[String],
) -> bool {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_containers(&dir, identifiers) {
            return true;
        }
    }
    false
}

fn check_dir_containers(
    dir: &std::path::Path,
    identifiers: &[String],
) -> bool {
    if let Ok(fp) = FilePath::new(dir.to_str().unwrap_or("")) {
        let entries = utility_file_cache::read_dir(&fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if path.is_dir() {
                if check_dir_containers(path, identifiers) {
                    return true;
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js")
                {
                    let fp = FilePath {
                        value: entry_path.value.clone(),
                    };
                    let content = utility_file_cache::read_cached(&fp).value;
                    for id in identifiers {
                        if content.contains(id) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
