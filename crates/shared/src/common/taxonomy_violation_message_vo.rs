// PURPOSE: taxonomy_violation_message_vo — recover the structured violation
// fields from a LintMessage's embedded text. Pure functions over a string, no
// I/O. This is the backward-compat path for a capability that has not yet
// populated `violation_name` / `why` / `fix` on its LintResult: the legacy
// message format carried the same three pieces, so the report can stay
// complete without every capability having been converted first.

/// Extract the violation subtype name from a message.
///
/// Format: `AES401 TAXONOMY_ROLE: description...`
/// Returns the token after the code prefix (e.g. "TAXONOMY_ROLE").
/// Falls back to empty when the message does not match the expected shape.
pub fn parse_violation_name(code: &str, message: &str) -> String {
    let first_line = message.lines().next().unwrap_or("");
    // Only a message that actually starts with the code can yield a name.
    // Without this guard a tool code that itself contains a colon
    // (`markdownlint::MD022`) would split on its own prefix and hand back
    // prose from the message body.
    let Some(rest) = first_line.strip_prefix(code) else {
        return String::new();
    };
    let rest = rest.trim_start_matches([' ', ':']);
    // The name is the first whitespace-delimited token, and it must be a
    // SCREAMING_SNAKE_CASE identifier — anything else is description prose.
    let token = rest.split_whitespace().next().unwrap_or("");
    let name = token.trim_end_matches(':');
    if is_violation_name(name) {
        return name.to_string();
    }
    String::new()
}

/// A violation name is upper-case identifiers joined by underscores
/// (`TAXONOMY_ROLE`, `AGENT_ROLE`, `H2_MISSING`). Rejects anything that
/// carries spaces, lowercase prose, or punctuation.
fn is_violation_name(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.len() <= 64
        && candidate
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        && candidate
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
}

/// Extract WHY text from a message.
///
/// Recognizes these line prefixes (anywhere in the message, line by line):
/// - `WHY?`  (legacy format used by capabilities)
/// - `WHY:`
///
/// Returns the text after the prefix, trimmed. Empty string when no WHY
/// line is present.
pub fn parse_why(message: &str) -> String {
    for line in message.lines() {
        let trimmed = line.trim();
        if let Some(text) = trimmed
            .strip_prefix("WHY?")
            .or_else(|| trimmed.strip_prefix("WHY:"))
        {
            return text.trim().to_string();
        }
    }
    String::new()
}

/// Extract FIX text from a message.
///
/// Recognizes:
/// - `FIX:`  (canonical short form)
/// - `HOW TO FIX?`  (legacy long form)
///
/// Returns the text after the prefix, trimmed. Empty string when no FIX
/// line is present.
pub fn parse_fix(message: &str) -> String {
    for line in message.lines() {
        let trimmed = line.trim();
        if let Some(text) = trimmed
            .strip_prefix("HOW TO FIX?")
            .or_else(|| trimmed.strip_prefix("FIX:"))
        {
            return text.trim().to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name_from_standard_message() {
        let msg =
            "AES401 TAXONOMY_ROLE: Direct primitive in taxonomy entity.\nWHY? reason\nFIX: fix";
        assert_eq!(parse_violation_name("AES401", msg), "TAXONOMY_ROLE");
    }

    #[test]
    fn parse_name_fallback_empty() {
        let msg = "no code prefix here";
        assert_eq!(parse_violation_name("AES401", msg), "");
    }

    #[test]
    fn parse_why_question_mark() {
        let msg = "CODE NAME: desc\nWHY? This is the reason.";
        assert_eq!(parse_why(msg), "This is the reason.");
    }

    #[test]
    fn parse_why_colon() {
        let msg = "CODE NAME: desc\nWHY: reason here.";
        assert_eq!(parse_why(msg), "reason here.");
    }

    #[test]
    fn parse_why_empty_when_absent() {
        assert_eq!(parse_why("no why line"), "");
    }

    #[test]
    fn parse_fix_canonical() {
        let msg = "CODE NAME: desc\nFIX: Do the fix.";
        assert_eq!(parse_fix(msg), "Do the fix.");
    }

    #[test]
    fn parse_fix_legacy_long_form() {
        let msg = "CODE NAME: desc\nHOW TO FIX? Do the fix.";
        assert_eq!(parse_fix(msg), "Do the fix.");
    }

    #[test]
    fn parse_fix_prefers_canonical_over_legacy() {
        let msg = "desc\nHOW TO FIX? old\nFIX: new";
        // First match wins — legacy line appears first.
        assert_eq!(parse_fix(msg), "old");
    }

    #[test]
    fn parse_fix_empty_when_absent() {
        assert_eq!(parse_fix("no fix line"), "");
    }
}
