// PURPOSE: Pure filename utility functions for orphan detection (AES layer naming)
// These are stateless, domain-agnostic, reusable across multiple capabilities.

use shared::orphan_rules::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

/// Identify entry points from file list using configured patterns.
/// Pure function — no state, no I/O.
pub fn identify_entry_points(
    files: &[OrphanFileListVO],
    configured: &[OrphanEntryPatternListVO],
) -> OrphanFileListVO {
    let file_strs: Vec<String> = files
        .iter()
        .flat_map(|v| v.values.iter().cloned())
        .collect();
    let configured_strs: Vec<String> = configured
        .iter()
        .flat_map(|p| p.values.iter().cloned())
        .collect();

    let matched: Vec<String> = if configured_strs.is_empty() {
        file_strs
            .iter()
            .filter(|f| {
                let basename = f.rsplit('/').next().unwrap_or(f);
                // Entry points: only files with _entry suffix
                basename.ends_with("_entry.rs")
                    || basename.ends_with("_entry.py")
                    || basename.ends_with("_entry.ts")
                    || basename.ends_with("_entry.js")
            })
            .cloned()
            .collect()
    } else {
        file_strs
            .iter()
            .filter(|f| {
                let basename = f.rsplit('/').next().unwrap_or(f);
                let stem = file_stem(basename);
                configured_strs.iter().any(|pattern| {
                    basename == pattern
                        || stem == *pattern
                        || (pattern.starts_with('_') && stem.ends_with(pattern.as_str()))
                        || (pattern.starts_with('.') && basename.ends_with(pattern.as_str()))
                        || (pattern == "root_" && basename.starts_with("root_"))
                        || (pattern.ends_with(".rs")
                            || pattern.ends_with(".py")
                            || pattern.ends_with(".ts")
                            || pattern.ends_with(".js"))
                            && basename.ends_with(pattern.as_str())
                })
            })
            .cloned()
            .collect()
    };
    let mut matched = matched;
    matched.sort();
    matched.dedup();
    OrphanFileListVO::new(matched)
}

/// Extract basename from path: "crates/shared/src/lib.rs" -> "lib.rs"
pub fn file_basename(path: &str) -> String {
    match path.rsplit('/').next() {
        Some(f) => f.to_string(),
        None => path.to_string(),
    }
}

/// Extract stem from path: "checker.rs" -> "checker", "capabilities_checker.rs" -> "capabilities_checker"
pub fn file_stem(path: &str) -> String {
    let base = file_basename(path);
    if let Some(pos) = base.rfind('.') {
        base[..pos].to_string()
    } else {
        base
    }
}

/// Extract suffix after last underscore in stem: "capabilities_checker.rs" -> "checker"
pub fn file_suffix(path: &str) -> String {
    let st = file_stem(path);
    match st.rfind('_') {
        Some(pos) => st[pos + 1..].to_string(),
        None => String::new(),
    }
}

/// Check if a whole word exists in the text, ignoring punctuation and whitespace.
/// Used for identifier matching in contract and agent orphan detection.
/// Pure function — no state, no I/O.
pub fn content_contains_whole_word(text: &str, word: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|w| w == word)
}
