// PURPOSE: Stateless path resolution utilities

use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    crate::common::utility_file_handler::find_workspace_root(path)
}

/// Detect ConfigLanguage from a file system path by checking for workspace type markers in the path.
/// Checks parent directory names (`crates`, `modules`, `packages`) and the path itself.
pub fn detect_language_from_path(path: &str) -> ConfigLanguage {
    let path_buf = std::path::PathBuf::from(path);

    if crate::common::utility_file_handler::path_exists(path_buf.join("Cargo.toml"))
        || path_contains_component(&path_buf, "crates")
    {
        return ConfigLanguage::Rust;
    }
    if crate::common::utility_file_handler::path_exists(path_buf.join("package.json"))
        || path_contains_component(&path_buf, "packages")
    {
        return ConfigLanguage::TypeScript;
    }
    if crate::common::utility_file_handler::path_exists(path_buf.join("pyproject.toml"))
        || crate::common::utility_file_handler::path_exists(path_buf.join("setup.py"))
        || crate::common::utility_file_handler::path_exists(path_buf.join("requirements.txt"))
        || path_contains_component(&path_buf, "modules")
    {
        return ConfigLanguage::Python;
    }

    ConfigLanguage::Rust
}

fn path_contains_component(path: &std::path::Path, component: &str) -> bool {
    path.components()
        .any(|c| matches!(c, std::path::Component::Normal(name) if name == component))
}

/// Extract workspace member name from a file path relative to the scan root.
/// e.g. `("test-workspaces/crates/shared_common/src/foo.rs", "test-workspaces/crates")` → `"shared_common"`
pub fn extract_member_from_path(file_path: &str, root: &str) -> String {
    let normalized_root = root.trim_end_matches('/');
    let normalized_path = file_path.trim_start_matches("./");

    // Skip common source directory names — they are not workspace members
    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];

    if let Some(rest) = normalized_path.strip_prefix(normalized_root) {
        let rest = rest.trim_start_matches('/');
        if let Some(member) = rest.split('/').next() {
            if !member.is_empty() && !skip_dirs.contains(&member) {
                return member.to_string();
            }
            // If the first component is a skip dir, go one level deeper
            if skip_dirs.contains(&member) {
                let deeper = rest.trim_start_matches('/').trim_start_matches(member).trim_start_matches('/');
                if let Some(real_member) = deeper.split('/').next() {
                    if !real_member.is_empty() && !skip_dirs.contains(&real_member) {
                        return real_member.to_string();
                    }
                }
            }
        }
    }
    for marker in &["crates", "modules", "packages"] {
        if let Some(idx) = normalized_path.find(marker) {
            let after = &normalized_path[idx + marker.len()..].trim_start_matches('/');
            if let Some(member) = after.split('/').next() {
                if !member.is_empty() && !skip_dirs.contains(&member) {
                    return member.to_string();
                }
            }
        }
    }
    ".".to_string()
}

/// Detect if a path is a member directory (not a workspace root).
/// Returns true only if the path itself has a Cargo.toml without [workspace],
/// meaning it's a single crate member, not a multi-member workspace or a directory container.
pub fn is_member_path(path: &str) -> bool {
    let p = std::path::Path::new(path);

    // If path itself has Cargo.toml without [workspace], it's a member crate
    let cargo_toml = p.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            return !content.contains("[workspace]");
        }
        return true;
    }

    false
}
