use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct UtilityRoleChecker {}

impl IUtilityRoleChecker for UtilityRoleChecker {
    fn check_utility_convention(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();

        // Strip comments and macro bodies before checking for struct/enum definitions
        let stripped = strip_comments_and_macros(content);

        if stripped.contains("pub struct ") || stripped.contains("pub enum ") {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::MEDIUM,
                AesRoleViolation::UtilityRole {
                    reason: Some("Utility files must not define structs or enums.".into()),
                }
                .to_string(),
            ));
        }
    }
}

/// Strip comments and macro bodies from Rust source code.
/// This prevents false positives when `pub struct` appears inside
/// a macro definition or a comment.
fn strip_comments_and_macros(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_macro = false;
    let mut brace_depth = 0;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        // Handle block comments
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }

        // Handle line comments
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                result.push(c);
            }
            continue;
        }

        // Handle macro bodies
        if in_macro {
            if c == '{' {
                brace_depth += 1;
            } else if c == '}' {
                brace_depth -= 1;
                if brace_depth == 0 {
                    in_macro = false;
                }
            }
            continue;
        }

        // Check for comment start
        if c == '/' && chars.peek() == Some(&'/') {
            in_line_comment = true;
            chars.next();
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            in_block_comment = true;
            chars.next();
            continue;
        }

        // Check for macro_rules! start
        if c == 'm' {
            let remaining: String = chars.clone().take(11).collect();
            if remaining.starts_with("acro_rules!") {
                // Skip "acro_rules!" (11 chars)
                for _ in 0..11 {
                    chars.next();
                }
                // Skip whitespace and find opening brace
                while let Some(&next_c) = chars.peek() {
                    if next_c == '{' {
                        in_macro = true;
                        brace_depth = 1;
                        chars.next();
                        break;
                    }
                    chars.next();
                }
                continue;
            }
        }

        result.push(c);
    }

    result
}

impl Default for UtilityRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}
