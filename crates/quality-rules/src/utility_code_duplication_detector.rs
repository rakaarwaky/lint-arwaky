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
