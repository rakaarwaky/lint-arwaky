// PURPOSE: Stateless path resolution utilities (business logic only)
// Filesystem functions: see filesystem::utility_filesystem_io

use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;

/// Detect ConfigLanguage from a file system path by checking for workspace type markers in the path.
pub fn detect_language_from_path(path: &str) -> ConfigLanguage {
    let path_buf = std::path::PathBuf::from(path);

    if crate::filesystem::utility_filesystem_io::path_exists(path_buf.join("Cargo.toml"))
        || path_contains_component(&path_buf, "crates")
    {
        return ConfigLanguage::Rust;
    }
    if crate::filesystem::utility_filesystem_io::path_exists(path_buf.join("package.json"))
        || path_contains_component(&path_buf, "packages")
    {
        return ConfigLanguage::TypeScript;
    }
    if crate::filesystem::utility_filesystem_io::path_exists(path_buf.join("pyproject.toml"))
        || crate::filesystem::utility_filesystem_io::path_exists(path_buf.join("setup.py"))
        || crate::filesystem::utility_filesystem_io::path_exists(path_buf.join("requirements.txt"))
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
pub fn extract_member_from_path(file_path: &str, root: &str) -> String {
    let normalized_root = root.trim_end_matches('/');
    let normalized_path = file_path.trim_start_matches("./");

    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];

    if let Some(rest) = normalized_path.strip_prefix(normalized_root) {
        let rest = rest.trim_start_matches('/');
        if let Some(member) = rest.split('/').next() {
            if !member.is_empty() && !skip_dirs.contains(&member) {
                return member.to_string();
            }
            if skip_dirs.contains(&member) {
                let deeper = rest
                    .trim_start_matches('/')
                    .trim_start_matches(member)
                    .trim_start_matches('/');
                if let Some(real_member) = deeper.split('/').next()
                    && !real_member.is_empty()
                    && !skip_dirs.contains(&real_member)
                {
                    if real_member.contains('.')
                        && let Some(root_member) = normalized_root.rsplit('/').next()
                        && !root_member.is_empty()
                    {
                        return root_member.to_string();
                    }
                    return real_member.to_string();
                }
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
