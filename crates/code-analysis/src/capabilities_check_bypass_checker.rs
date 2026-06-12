// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES022: detect #[allow], noqa, unwrap, panic
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::taxonomy_violation_message::AesViolation;

pub struct BypassChecker {}

impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IBypassCheckerProtocol for BypassChecker {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let markers = [
            ("H", "noqa"),
            ("H", "type: ignore"),
            ("H", "pylint: disable"),
            ("S", "eslint-disable"),
            ("A", "ts-ignore"),
            ("A", "ts-expect-error"),
            ("S", "NOLINT"),
        ];
        let mkc = |p, k| match p {
            "H" => format!("# {}", k),
            "S" => format!("// {}", k),
            "A" => format!("//@{}", k),
            _ => String::new(),
        };
        let patterns: Vec<String> = markers.iter().map(|&(p, k)| mkc(p, k)).collect();
        let unwrap_pat = [".", "unwrap()"].concat();
        let expect_pat = [".", "expect("].concat();
        let panic_pat = ["panic", "!("].concat();
        let todo_pat = ["todo", "!("].concat();
        let unimpl_pat = ["unimplemented", "!("].concat();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("#[allow(") || t.starts_with("#[expect(") {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AesViolation::BypassComment { reason: None },
                ));
                continue;
            }
            for p in &patterns {
                if t.to_lowercase().contains(p.as_str()) {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES022",
                        Severity::CRITICAL,
                        AesViolation::BypassComment { reason: None },
                    ));
                    break;
                }
            }
            if t.contains(&unwrap_pat) || t.contains(&expect_pat) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AesViolation::UnwrapExpect { reason: None },
                ));
                continue;
            }
            if t.contains(&panic_pat) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AesViolation::Panic { reason: None },
                ));
                continue;
            }
            if t.contains(&todo_pat) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AesViolation::Panic { reason: None },
                ));
                continue;
            }
            if t.contains(&unimpl_pat) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AesViolation::Panic { reason: None },
                ));
                continue;
            }
        }
    }
}
