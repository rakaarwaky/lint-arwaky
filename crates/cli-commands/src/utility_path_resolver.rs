// PURPOSE: Stateless path resolution utilities

use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::filesystem::utility_filesystem_io::find_workspace_root(path)
}

/// Detect ConfigLanguage from a file system path by checking for workspace type markers in the path.
/// Checks parent directory names (`crates`, `modules`, `packages`) and the path itself.
pub fn detect_language_from_path(path: &str) -> ConfigLanguage {
    let path_buf = std::path::PathBuf::from(path);

    if shared::filesystem::utility_filesystem_io::path_exists(path_buf.join("Cargo.toml"))
        || path_contains_component(&path_buf, "crates")
    {
        return ConfigLanguage::Rust;
    }
    if shared::filesystem::utility_filesystem_io::path_exists(path_buf.join("package.json"))
        || path_contains_component(&path_buf, "packages")
    {
        return ConfigLanguage::TypeScript;
    }
    if shared::filesystem::utility_filesystem_io::path_exists(path_buf.join("pyproject.toml"))
        || shared::filesystem::utility_filesystem_io::path_exists(path_buf.join("setup.py"))
        || shared::filesystem::utility_filesystem_io::path_exists(path_buf.join("requirements.txt"))
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
                let deeper = rest
                    .trim_start_matches('/')
                    .trim_start_matches(member)
                    .trim_start_matches('/');
                if let Some(real_member) = deeper.split('/').next()
                    && !real_member.is_empty()
                    && !skip_dirs.contains(&real_member)
                {
                    // If it has a file extension, it's a file — the root IS the member
                    if real_member.contains('.')
                        && let Some(root_member) = normalized_root.rsplit('/').next()
                        && !root_member.is_empty()
                    {
                        return root_member.to_string();
                    }
                    return real_member.to_string();
                }
                // Nothing meaningful after skip dir — use root's last component
                if let Some(root_member) = normalized_root.rsplit('/').next()
                    && !root_member.is_empty()
                {
                    return root_member.to_string();
                }
            }
        }
    }
    for marker in &["crates", "modules", "packages"] {
        if let Some(idx) = normalized_path.find(marker) {
            let after = &normalized_path[idx + marker.len()..].trim_start_matches('/');
            if let Some(member) = after.split('/').next()
                && !member.is_empty()
                && !skip_dirs.contains(&member)
            {
                return member.to_string();
            }
        }
    }
    ".".to_string()
}

/// Detect if a path is a leaf member directory (not a workspace root and not a group of members).
/// A leaf member has a marker file AND does NOT contain subdirectories that are also members.
/// Skips common source directories (src, lib, bin, tests, benches, examples) to avoid
/// false negatives when a member's src/ contains __init__.py.
pub fn is_leaf_member_path(path: &str) -> bool {
    if !is_member_path(path) {
        return false;
    }
    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];
    let p = std::path::Path::new(path);
    if let Ok(entries) = std::fs::read_dir(p) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if skip_dirs.contains(&dir_name.as_str()) {
                    continue;
                }
                let sub_path = entry.path();
                if is_member_path(&sub_path.to_string_lossy()) {
                    return false;
                }
            }
        }
    }
    true
}

/// Detect if a path is a member directory (not a workspace root).
/// Returns true if the path is a single crate/module/package member:
/// - Rust: Cargo.toml without [workspace]
/// - Python: __init__.py or pyproject.toml present
/// - TypeScript: package.json present
pub fn is_member_path(path: &str) -> bool {
    let p = std::path::Path::new(path);

    // Rust: Cargo.toml without [workspace] → single crate member
    let cargo_toml = p.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            return !content.contains("[workspace]");
        }
        return true;
    }

    // Python: __init__.py or pyproject.toml → module member
    if p.join("__init__.py").exists() || p.join("pyproject.toml").exists() {
        return true;
    }

    // TypeScript: package.json → package member
    if p.join("package.json").exists() {
        return true;
    }

    false
}
