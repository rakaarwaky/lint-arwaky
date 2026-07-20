// PURPOSE: Pure filename utility functions for orphan detection (AES layer naming)
// These are stateless, domain-agnostic, reusable across multiple capabilities.

/// Extract basename from path: "crates/shared/src/lib.rs" → "lib.rs"
pub fn file_basename(path: &str) -> String {
    match path.rsplit('/').next() {
        Some(f) => f.to_string(),
        None => path.to_string(),
    }
}

/// Extract stem from path: "checker.rs" → "checker", "capabilities_checker.rs" → "capabilities_checker"
pub fn file_stem(path: &str) -> String {
    let base = file_basename(path);
    if let Some(pos) = base.rfind('.') {
        base[..pos].to_string()
    } else {
        base
    }
}

/// Extract suffix after last underscore in stem: "capabilities_checker.rs" → "checker"
pub fn file_suffix(path: &str) -> String {
    let st = file_stem(path);
    match st.rfind('_') {
        Some(pos) => st[pos + 1..].to_string(),
        None => String::new(),
    }
}
