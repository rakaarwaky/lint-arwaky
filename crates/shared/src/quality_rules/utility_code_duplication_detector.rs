// PURPOSE: Stateless utility functions for code duplication normalization (AES305)
// Only normalization helpers used by capabilities_code_duplication_analyzer remain here.

/// Normalize a single line: trim, keep only alphanumeric and whitespace.
pub fn normalize_line(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Normalize a window of lines into a single hash key.
pub fn normalize_window(window: &[&str]) -> String {
    window
        .iter()
        .map(|s| normalize_line(s))
        .collect::<Vec<_>>()
        .join("|")
}

/// True when a trimmed line is an import/`use` statement in Rust, Python,
/// TypeScript, or JavaScript. Import lines are mechanical scaffolding shared
/// by every file that depends on the same crates, not duplicated logic — so
/// AES305 must not count them toward the similarity percentage.
pub fn is_import_line(trimmed: &str) -> bool {
    if trimmed.is_empty() {
        return false;
    }
    let head = trimmed.as_bytes()[0];
    // Rust: `use` / `pub use`
    if trimmed.starts_with("use ")
        || trimmed.starts_with("use\t")
        || trimmed.starts_with("pub use ")
        || trimmed.starts_with("pub use\t")
    {
        return true;
    }
    // Python: `import` / `from ... import`
    if trimmed.starts_with("import ")
        || trimmed.starts_with("import\t")
        || trimmed.starts_with("from ")
        || trimmed.starts_with("from\t")
    {
        return true;
    }
    // TypeScript / JavaScript: `import` / `export ... from`
    if (head == b'i' || head == b'e')
        && (trimmed.starts_with("import ")
            || trimmed.starts_with("import\t")
            || trimmed.starts_with("export ") && trimmed.starts_with("export"))
    {
        if trimmed.starts_with("import ") || trimmed.starts_with("import\t") {
            return true;
        }
        // `export { ... } from ...` is a re-export, still scaffolding
        if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            return true;
        }
    }
    false
}
