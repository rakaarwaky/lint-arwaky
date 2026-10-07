// PURPOSE: Shared forbidden-I/O scan helper for agent role auditors.
//
// This module owns the language-agnostic line scan used by the Python, Rust,
// and TypeScript agent auditors to detect forbidden I/O tokens. Keeping it in
// a dedicated utility file also preserves the `shared/role_rules` utility
// module size boundary.

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;

/// True when a line is a comment in any of the three languages.
pub fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("///")
}

/// Scan `content` for forbidden I/O tokens from `tokens` and push a
/// `LintResult` per hit. Replaces the identical loop repeated in
/// `AgentPythonRoleAuditor`, `AgentRustRoleAuditor`, and
/// `AgentTsRoleAuditor` — only the token table and the language label
/// in the message differ.
///
/// `language_label` is the language name embedded in the fix hint
/// (e.g. "Python", "Rust", "TypeScript").
pub fn scan_io_forbidden(
    content: &str,
    path: &str,
    language_label: &str,
    tokens: &[(&str, &str)],
    violations: &mut Vec<LintResult>,
) {
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if is_comment(t) {
            continue;
        }
        for (token, what) in tokens {
            if t.contains(token) {
                violations.push(LintResult::new_arch(
                    path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!(
                        "AES405 AGENT_ROLE: Forbidden {what} in a {language_label} agent file.\n\
                         WHY? Line {} uses `{token}`. An agent coordinates in-memory \
                         protocols and must not perform I/O itself.\n\
                         FIX: Move the {what} into a capability or surface module and \
                         inject it via a protocol.",
                        i + 1
                    ),
                ));
            }
        }
    }
}
