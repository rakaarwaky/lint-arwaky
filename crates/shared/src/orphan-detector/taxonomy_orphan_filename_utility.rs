// PURPOSE: Pure filename utility functions for orphan detection (AES layer naming)
// These are stateless, domain-agnostic, reusable across multiple capabilities.

/// Extract basename from path: "crates/shared/src/lib.rs" → "lib.rs"
pub fn file_basename(path: &str) -> String {
    match path.rsplit('/').next() {
        Some(f) => f.to_string(),
        None => path.to_string(),
    }
}

/// Extract stem from basename: "checker.rs" → "checker", "lib.rs" → "lib"
pub fn file_stem(path: &str) -> String {
    let base = file_basename(path);
    match base.rsplit_once('.') {
        Some((stem, _)) => stem.to_string(),
        None => base,
    }
}

/// Extract suffix after last underscore: "capabilities_checker.rs" → "checker"
pub fn file_suffix(path: &str) -> String {
    let base = file_basename(path);
    match base.rsplit_once('_') {
        Some((_, suffix)) => suffix.to_string(),
        None => base,
    }
}
