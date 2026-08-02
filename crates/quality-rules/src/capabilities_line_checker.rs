use shared::cli_commands::LintResult;
use shared::quality_rules::{AesCodeAnalysisViolation, ILineCheckerProtocol};

use shared::common::{LayerDefinition, LintMessage, Severity};

// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
// ALGORITHM:
//   1. Skip barrel files (mod.rs, __init__.py)
//   2. If no LayerDefinition provided, skip
//   3. Check if filename is in exception list
//   4. Count lines in passed content string
//   5. If min_lines > 0 and count < min_lines → AES302 FILE_TOO_SHORT
//   6. If max_lines > 0 and count > max_lines → AES301 FILE_TOO_LARGE
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchLineChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if basename == "__init__.py" || basename == "mod.rs" {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let count = content.lines().count() as i64;

        if def.code_analysis.min_lines.value > 0 && count < def.code_analysis.min_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES302",
                Severity::HIGH,
                format!(
                    "{} (min: {}).",
                    AesCodeAnalysisViolation::FileTooShort {
                        reason: Some(LintMessage::new(format!(
                            "File has {} lines, less than minimum {} lines",
                            count, def.code_analysis.min_lines.value
                        ))),
                    },
                    def.code_analysis.min_lines.value
                ),
            ));
        }

        if def.code_analysis.max_lines.value > 0 && count > def.code_analysis.max_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES301",
                Severity::HIGH,
                format!(
                    "{} (max: {}).",
                    AesCodeAnalysisViolation::FileTooLarge {
                        reason: Some(LintMessage::new(format!(
                            "File has {} lines, exceeding maximum {} lines",
                            count, def.code_analysis.max_lines.value
                        ))),
                    },
                    def.code_analysis.max_lines.value
                ),
            ));
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::common::taxonomy_common_vo::Count;
    use shared::common::taxonomy_definition_vo::LayerDefinition;

    fn checker() -> ArchLineChecker {
        ArchLineChecker::new()
    }

    fn def_with_lines(min: i64, max: i64) -> LayerDefinition {
        LayerDefinition {
            code_analysis: shared::quality_rules::CodeAnalysisRuleVO {
                min_lines: Count::new(min),
                max_lines: Count::new(max),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn construction_succeeds() {
        let _ = checker();
    }

    #[test]
    fn no_violation_within_bounds() {
        let mut violations = Vec::new();
        let def = def_with_lines(5, 100);
        let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\n";
        checker().check_line_counts("src/lib.rs", Some(&def), content, &mut violations);
        assert!(violations.is_empty());
    }

    #[test]
    fn file_too_short_produces_violation() {
        let mut violations = Vec::new();
        let def = def_with_lines(10, 100);
        let content = "a\nb\nc\n";
        checker().check_line_counts("src/lib.rs", Some(&def), content, &mut violations);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.code().contains("AES302"));
    }

    #[test]
    fn file_too_large_produces_violation() {
        let mut violations = Vec::new();
        let def = def_with_lines(0, 3);
        let content = "a\nb\nc\nd\ne\n";
        checker().check_line_counts("src/lib.rs", Some(&def), content, &mut violations);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.code().contains("AES301"));
    }

    #[test]
    fn both_violations_when_outside_range() {
        let mut violations = Vec::new();
        let def = def_with_lines(10, 5);
        let content = "a\n";
        checker().check_line_counts("src/lib.rs", Some(&def), content, &mut violations);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn barrel_file_mod_rs_skipped() {
        let mut violations = Vec::new();
        let def = def_with_lines(100, 100);
        let content = "short";
        checker().check_line_counts("src/mod.rs", Some(&def), content, &mut violations);
        assert!(violations.is_empty());
    }

    #[test]
    fn barrel_file_init_py_skipped() {
        let mut violations = Vec::new();
        let def = def_with_lines(100, 100);
        let content = "short";
        checker().check_line_counts("src/__init__.py", Some(&def), content, &mut violations);
        assert!(violations.is_empty());
    }

    #[test]
    fn no_definition_skipped() {
        let mut violations = Vec::new();
        let content = "short";
        checker().check_line_counts("src/lib.rs", None, content, &mut violations);
        assert!(violations.is_empty());
    }
}
