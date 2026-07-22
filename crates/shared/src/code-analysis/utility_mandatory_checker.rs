// PURPOSE: Stateless utility functions for mandatory definition checking (AES303)
// Extracted from capabilities_mandatory_definition_checker.rs — pure functions, no &self, no I/O

use super::utility_bypass_detector::matches_keyword_token;

/// Check if a line declares a Rust struct/enum/trait/type using word-boundary matching.
/// Handles visibility modifiers (pub, pub(crate)), tuple structs, and avoids
/// substring false-positives like "obstruction", "structure", "instruction".
pub fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait", "type"];
    for kw in keywords {
        if matches_keyword_token(line, kw) {
            return true;
        }
    }
    false
}
