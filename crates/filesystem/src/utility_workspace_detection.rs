// FR-005: Workspace Detection
// Produces: workspace root, member status, source dir, language
// Consumers: cli-commands, external-lint, orphan-detector, config-system
//
// Utility: stateless standalone functions

use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use std::path::{Path, PathBuf};

// ─── IO primitives delegated to shared (AES201: utility cannot import utility) ───

fn read_to_string(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn list_dir_entries(dir: &Path) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir.read_dir() {
        for entry in read_dir.flatten() {
            entries.push(entry.path());
        }
    }
    entries
}

fn canonicalize(path: &Path) -> Result<PathBuf, std::io::Error> {
    std::fs::canonicalize(path)
}

// ═══════════════════════════════════════════════════════════════
// Workspace Root Detection
// ═══════════════════════════════════════════════════════════════

/// Find workspace root by walking up from start path.
pub fn find_workspace_root(start: &str) -> Option<PathBuf> {
    let mut dir = Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if dir.join("Cargo.toml").exists() {
            if let Some(parent) = dir.parent() {
                let parent_name = parent
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if parent.join("crates").is_dir()
                    || parent.join("packages").is_dir()
                    || parent.join("modules").is_dir()
                    || matches!(parent_name, "crates" | "packages" | "modules")
                {
                    // Don't return yet — parent is the real workspace root
                } else {
                    return Some(dir);
                }
            } else {
                return Some(dir);
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Find workspace root (Result variant).
pub fn find_workspace_root_from_path(start: &Path) -> Result<PathBuf, std::io::Error> {
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

// ═══════════════════════════════════════════════════════════════
// Member Detection
// ═══════════════════════════════════════════════════════════════

/// Detect if a path is a single workspace member.
pub fn is_member_path(path: &str) -> bool {
    let p = Path::new(path);

    // Rust: Cargo.toml without [workspace]
    let cargo_toml = p.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = read_to_string(&cargo_toml) {
            return !content.contains("[workspace]");
        }
        return true;
    }

    // Python: __init__.py or pyproject.toml
    if p.join("__init__.py").exists() || p.join("pyproject.toml").exists() {
        return true;
    }

    // TypeScript: package.json
    if p.join("package.json").exists() {
        return true;
    }

    false
}

/// Detect if a path is a leaf member (not a group of members).
pub fn is_leaf_member_path(path: &str) -> bool {
    if !is_member_path(path) {
        return false;
    }
    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];
    let p = Path::new(path);
    for entry_path in list_dir_entries(p) {
        let name = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if entry_path.is_dir() {
            if skip_dirs.contains(&name) {
                continue;
            }
            if is_member_path(&entry_path.to_string_lossy()) {
                return false;
            }
        }
    }
    true
}

// ═══════════════════════════════════════════════════════════════
// Source Directory Detection
// ═══════════════════════════════════════════════════════════════

/// Detect source directory from project root.
pub fn detect_source_dir(project_root: &Path) -> PathBuf {
    if has_source_files(project_root) {
        return project_root.to_path_buf();
    }
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
}

fn has_source_files(dir: &Path) -> bool {
    for entry_path in list_dir_entries(dir) {
        if let Some(name) = entry_path.file_name().and_then(|n| n.to_str())
            && (name.ends_with(".rs")
                || name.ends_with(".py")
                || name.ends_with(".ts")
                || name.ends_with(".js"))
        {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════
// Language Detection
// ═══════════════════════════════════════════════════════════════

/// Detect ConfigLanguage from a file system path.
pub fn detect_language_from_path(path: &str) -> ConfigLanguage {
    let path_buf = std::path::PathBuf::from(path);

    if path_buf.join("Cargo.toml").exists() || path_contains_component(&path_buf, "crates") {
        return ConfigLanguage::Rust;
    }
    if path_buf.join("package.json").exists() || path_contains_component(&path_buf, "packages") {
        return ConfigLanguage::TypeScript;
    }
    if path_buf.join("pyproject.toml").exists()
        || path_buf.join("setup.py").exists()
        || path_buf.join("requirements.txt").exists()
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

/// Detect languages by walking directory tree.
/// Returns (has_rust, has_python, has_js).
pub fn detect_languages(root: &std::path::Path) -> (bool, bool, bool) {
    let mut has_rs = false;
    let mut has_py = false;
    let mut has_js = false;

    fn walk_detect(dir: &std::path::Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        for path in list_dir_entries(dir) {
            if path.is_dir() {
                let name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                if shared::common::DEFAULT_IGNORED_PATHS.contains(&name) {
                    continue;
                }
                walk_detect(&path, has_rs, has_py, has_js);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => *has_rs = true,
                    "py" => *has_py = true,
                    "js" | "ts" | "jsx" | "tsx" => *has_js = true,
                    _ => {}
                }
            }
            if *has_rs && *has_py && *has_js {
                return;
            }
        }
    }

    if root.is_file() {
        if let Some(ext) = root.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => has_rs = true,
                "py" => has_py = true,
                "js" | "ts" | "jsx" | "tsx" => has_js = true,
                _ => {}
            }
        }
    } else {
        walk_detect(root, &mut has_rs, &mut has_py, &mut has_js);
    }
    (has_rs, has_py, has_js)
}

// ═══════════════════════════════════════════════════════════════
// Path Utilities
// ═══════════════════════════════════════════════════════════════

/// Confine a candidate path under a root directory. Returns canonicalized path if valid.
pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = canonicalize(root).ok()?;
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };
    if let Ok(canonical_candidate) = canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;
    let canonical_parent = canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);
    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

/// Check if a directory contains files matching identifiers (recursive).
pub fn check_dir_containers(dir: &Path, identifiers: &[String]) -> bool {
    for path in list_dir_entries(dir) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if shared::common::DEFAULT_IGNORED_PATHS.contains(&name) {
            continue;
        }
        if path.is_dir() && check_dir_containers(&path, identifiers) {
            return true;
        } else if (name.ends_with("_container.rs")
            || name.ends_with("_container.py")
            || name.ends_with("_container.ts")
            || name == "lib.rs")
            && let Ok(content) = read_to_string(&path)
        {
            for id in identifiers {
                if content.contains(id) {
                    return true;
                }
            }
        }
    }
    false
}

/// Discover source files under root, skipping ignored directories during traversal.
/// Uses `ignore::WalkBuilder` for efficient directory skipping plus a post-walk
/// filter using `is_path_ignored` to handle all config-specified patterns.
/// Uses shared::common::DEFAULT_IGNORED_PATHS as built-in defaults.
pub fn discover_source_files(root: &Path, ignored: &[String]) -> Vec<String> {
    // Merge single-source defaults + caller-provided patterns
    let mut merged_ignored: Vec<String> = shared::common::DEFAULT_IGNORED_PATHS
        .iter()
        .map(|s| s.to_string())
        .collect();
    for pat in ignored {
        if !merged_ignored.contains(pat) {
            merged_ignored.push(pat.clone());
        }
    }

    let mut builder = ignore::WalkBuilder::new(root);
    builder.hidden(false).git_ignore(true);
    for pat in &merged_ignored {
        builder.add_ignore(pat.as_str());
    }
    let walker = builder.build();
    let exts: Vec<&str> = vec!["rs", "py", "js", "ts", "jsx", "tsx"];
    let abs_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| exts.contains(&ext))
                .unwrap_or(false)
        })
        .filter(|e| {
            let abs_path = e
                .path()
                .canonicalize()
                .unwrap_or_else(|_| e.path().to_path_buf());
            let rel_path = abs_path.strip_prefix(&abs_root).unwrap_or(e.path());
            let rel_str = rel_path.to_string_lossy();
            !shared::common::utility_path_filter::is_path_ignored(&rel_str, &merged_ignored)
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect()
}

/// Scan directory recursively for all paths as PathBuf.
pub fn scan_directory_paths(root: &Path) -> Vec<PathBuf> {
    let walker = ignore::WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .build();
    walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Scan directory recursively for all files as strings.
pub fn scan_directory(root: &Path) -> Vec<String> {
    scan_directory_paths(root)
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}

/// Discover all files (source + non-source) under root.
pub fn discover_files(root: &Path) -> Vec<String> {
    scan_directory(root)
}
