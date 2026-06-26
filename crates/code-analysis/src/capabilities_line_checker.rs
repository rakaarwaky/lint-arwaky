// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
// ALGORITHM:
//   1. Skip barrel files (mod.rs, __init__.py)
//   2. If no LayerDefinition provided, skip
//   3. Check if filename is in exception list
//   4. Count lines in passed content string
//   5. If min_lines > 0 and count < min_lines → AES302 FILE_TOO_SHORT
//   6. If max_lines > 0 and count > max_lines → AES301 FILE_TOO_LARGE
use std::path::Path;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct ArchLineChecker {}

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
                    AesCodeAnalysisViolation::FileTooShort { reason: None },
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
                    AesCodeAnalysisViolation::FileTooLarge { reason: None },
                    def.code_analysis.max_lines.value
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::taxonomy_common_vo::{Count, PatternList};
    use shared::taxonomy_definition_vo::LayerDefinition;

    fn make_def(min_lines: i64, max_lines: i64) -> Option<LayerDefinition> {
        Some(LayerDefinition {
            code_analysis: CodeAnalysisRuleVO {
                min_lines: Count::new(min_lines),
                max_lines: Count::new(max_lines),
                ..Default::default()
            },
            exceptions: PatternList::default(),
            ..Default::default()
        })
    }

    fn make_def_with_exception(
        min_lines: i64,
        max_lines: i64,
        exception: &str,
    ) -> Option<LayerDefinition> {
        Some(LayerDefinition {
            code_analysis: CodeAnalysisRuleVO {
                min_lines: Count::new(min_lines),
                max_lines: Count::new(max_lines),
                ..Default::default()
            },
            exceptions: PatternList::new(vec![exception.to_string()]),
            ..Default::default()
        })
    }

    #[test]
    fn line_checker_skips_mod_rs() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "src/mod.rs",
            make_def(5, 20).as_ref(),
            "a\nb\nc\nd\ne\nf\n",
            &mut violations,
        );
        assert!(violations.is_empty(), "mod.rs should be skipped");
    }

    #[test]
    fn line_checker_skips_init_py() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "src/__init__.py",
            make_def(5, 20).as_ref(),
            "a\nb\nc\nd\ne\nf\n",
            &mut violations,
        );
        assert!(violations.is_empty(), "__init__.py should be skipped");
    }

    #[test]
    fn line_checker_within_limits() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "src/test.rs",
            make_def(2, 10).as_ref(),
            "a\nb\nc\nd\ne\n",
            &mut violations,
        );
        assert!(
            violations.is_empty(),
            "5 lines within [2, 10] should be fine"
        );
    }

    #[test]
    fn line_checker_too_large() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "src/test.rs",
            make_def(2, 3).as_ref(),
            "a\nb\nc\nd\ne\nf\n",
            &mut violations,
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.to_string().contains("AES301"));
    }

    #[test]
    fn line_checker_too_short() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "src/test.rs",
            make_def(5, 10).as_ref(),
            "a\nb\nc\n",
            &mut violations,
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.to_string().contains("AES302"));
    }

    #[test]
    fn line_checker_skips_exceptions() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts(
            "generated.rs",
            make_def_with_exception(2, 3, "generated.rs").as_ref(),
            "a\nb\nc\nd\ne\nf\n",
            &mut violations,
        );
        assert!(
            violations.is_empty(),
            "exceptions should skip even if too large"
        );
    }

    #[test]
    fn line_checker_no_definition_no_check() {
        let checker = ArchLineChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts("src/test.rs", None, "a\nb\nc\n", &mut violations);
        assert!(violations.is_empty(), "no definition means no check");
    }
}
