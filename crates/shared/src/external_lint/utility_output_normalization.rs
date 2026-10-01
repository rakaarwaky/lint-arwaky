// PURPOSE: Output normalization utilities for external tool execution.
// Plain free functions — no protocol / dependency injection.

/// Strip ANSI escape sequences from captured tool output.
///
/// External tools decide whether to colorize from the environment they
/// inherit, not from whether their output is about to be parsed. On a CI
/// runner prettier emits its report prefix as `[\e[33mwarn\e[39m]`, so an
/// adapter matching the literal `[warn]` silently drops every finding and the
/// scan reports the tool as clean. The same hazard applies to tsc's
/// regex-matched lines and markdownlint's ` error ` delimiter.
///
/// Normalizing once, before any adapter parses, keeps each adapter
/// independent of color, TTY detection, `FORCE_COLOR`, and `NO_COLOR`.
///
/// Handles CSI sequences (`ESC [ … final-byte`), OSC strings terminated by
/// BEL or ST, and bare two-character escapes.
pub fn strip_ansi_escapes(raw: &str) -> String {
    if !raw.contains('\u{1b}') {
        return raw.to_string();
    }
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            // CSI: parameters/intermediates, terminated by a byte in 0x40..=0x7E.
            Some('[') => {
                chars.next();
                for c in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&c) {
                        break;
                    }
                }
            }
            // OSC: a string, terminated by BEL or ST (ESC \).
            Some(']') => {
                chars.next();
                while let Some(c) = chars.next() {
                    if c == '\u{7}' {
                        break;
                    }
                    if c == '\u{1b}' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                }
            }
            // Two-character escape: drop the escape and its single operand.
            Some(_) => {
                chars.next();
            }
            // Trailing ESC with nothing after it.
            None => {}
        }
    }
    out
}
