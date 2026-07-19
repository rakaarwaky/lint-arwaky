// PURPOSE: Stateless utility functions for mandatory definition checking (AES303)
// Extracted from capabilities_mandatory_definition_checker.rs — pure functions, no &self, no I/O

/// Check if a line declares a Rust struct/enum/trait (handles visibility modifiers).
pub fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait"];
    for kw in keywords {
        if line.contains(kw) && !line.contains('(') {
            return true;
        }
    }
    false
}
