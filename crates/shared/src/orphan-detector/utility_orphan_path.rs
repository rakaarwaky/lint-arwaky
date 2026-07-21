use std::path::{Component, Path, PathBuf};

pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let root = normalize_lexical(root);
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let normalized = normalize_lexical(&absolute);
    if normalized.starts_with(&root) {
        Some(normalized)
    } else {
        None
    }
}

pub fn resolve_module_path(root: &Path, base_dir: &Path, module_path: &str) -> Option<PathBuf> {
    let candidate = if Path::new(module_path).is_absolute() {
        PathBuf::from(module_path)
    } else {
        base_dir.join(module_path)
    };
    confine_under_root(root, &candidate)
}

pub fn is_path_ignored(file: &str, patterns: &[String]) -> bool {
    let file = file.replace('\\', "/");
    patterns.iter().any(|pattern| {
        let raw = pattern.replace('\\', "/");
        if raw.is_empty() {
            return false;
        }
        if file == raw || file.ends_with(&raw) {
            return true;
        }
        let normalized = raw.trim_start_matches('/');
        if normalized.is_empty() {
            return false;
        }
        file.starts_with(&format!("{normalized}/"))
            || file.contains(&format!("/{normalized}/"))
            || file.contains(&format!("/{normalized}"))
    })
}
