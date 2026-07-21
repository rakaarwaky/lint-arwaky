use std::fs;
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
    let canonical_root = fs::canonicalize(root).ok()?;

    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };

    // If the candidate exists, canonicalize it directly.
    if let Ok(canonical_candidate) = fs::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }

    // If the candidate does not exist yet, canonicalize the parent
    // and reattach the final component.
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;

    let canonical_parent = fs::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);

    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
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
